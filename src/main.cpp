#include <iostream>
#include <filesystem>
#include <vector>
#include <algorithm>
#include <cctype>

#include "string.h"

#ifdef _WIN32
#include <windows.h>
#include <shellapi.h>
#endif

extern "C"
{
#include "romfs.h"
#include "pfs0.h"
}

using namespace std;
using namespace std::filesystem;

string path_to_utf8(const path &p)
{
    auto u8str = p.u8string();
    return string(u8str.begin(), u8str.end());
}

const vector<string> FILES_TO_COPY = {
    "romfs.bin",
    "exefs.nsp",
    "rtld",
    "main",
    "main.npdm",
    "compat0",
    "compat1",
    "compat2",
    "compat3",
    "compat4",
    "compat5",
    "compat6",
    "compat7",
    "compat8",
    "compat9",
    "subsdk0",
    "subsdk1",
    "subsdk2",
    "subsdk3",
    "subsdk4",
    "subsdk5",
    "subsdk6",
    "subsdk7",
    "subsdk8",
    "subsdk9",
    "sdk",
    "config.ini",
    "icon.jpg",
};

void print_help();
int parse_args(const vector<string> &args, path &input_path, path &output_path, path &manifest_path);
int check_pathes(path &input_path, path &output_path, path &manifest_path);
void list_items(const path &dir_path, vector<path> &items);
bool in_array(const string &value, const vector<string> &array);

int main(int argc, char *argv[])
{
    vector<string> args;
#ifdef _WIN32
    int wargc;
    LPWSTR *wargv = CommandLineToArgvW(GetCommandLineW(), &wargc);
    if (wargv)
    {
        for (int i = 0; i < wargc; ++i)
        {
            int size_needed = WideCharToMultiByte(CP_UTF8, 0, wargv[i], -1, NULL, 0, NULL, NULL);
            string utf8_arg(size_needed - 1, 0);
            WideCharToMultiByte(CP_UTF8, 0, wargv[i], -1, &utf8_arg[0], size_needed, NULL, NULL);
            args.push_back(utf8_arg);
        }
        LocalFree(wargv);
    }
#else
    for (int i = 0; i < argc; ++i)
        args.push_back(argv[i]);
#endif

    path input_path = current_path();
    path output_path = "mod.msp";
    path manifest_path = "manifest";

    if (parse_args(args, input_path, output_path, manifest_path) == 1)
        return 0;

    input_path = absolute(input_path);
    output_path = absolute(output_path);
    manifest_path = absolute(manifest_path);

    if (check_pathes(input_path, output_path, manifest_path) == 1)
        return 0;

    path current_dir = current_path();
    path temp_dir = current_dir / "temp/";

    if (exists(temp_dir))
        remove_all(temp_dir);

    create_directory(temp_dir);
    copy(manifest_path, temp_dir / "manifest");

    vector<path> all_items;
    list_items(input_path, all_items);
    for (path item : all_items)
    {
        string name = item.filename().string();
        for_each(name.begin(), name.end(), [](char &c)
                 { c = tolower(c); });

        if (is_directory(item) && name == "romfs")
        {
            cout << "Found romfs directory. Building romfs.bin..." << endl;
            build_romfs_by_paths((char *)path_to_utf8(item).c_str(), (char *)path_to_utf8(temp_dir / "romfs.bin").c_str());
            continue;
        }

        if (in_array(name, FILES_TO_COPY) || (name.ends_with(".ips")))
        {
            cout << "Found " << name << ", copying..." << endl;
            copy(item, temp_dir / item.filename(), copy_options::overwrite_existing);
            continue;
        }
    }

    cout << "Building " << output_path << "..." << endl;
    build_pfs0((char *)path_to_utf8(temp_dir).c_str(), (char *)path_to_utf8(output_path).c_str());
    remove_all(temp_dir);
    cout << "Done!" << endl;

    return 0;
}

void printHelp()
{
    cout << "build_msp" << endl;
    cout << "A tool that helps to build .msp file mod" << endl;
    cout << endl;
    cout << "Usage Example:" << endl;
    cout << "\tbuild_msp -i mod_dir -m manifest -o mod.msp" << endl;
    cout << endl;
    cout << "Options:" << endl;
    cout << "\t-i, --input <INPUT>\t[default .]" << endl;
    cout << "\t-o, --output <OUTPUT>\t[default mod.msp]" << endl;
    cout << "\t-m, --manifest <MANIFEST>\t[default manifest]" << endl;
}

int parse_args(const vector<string> &args, path &input_path, path &output_path, path &manifest_path)
{
    for (size_t i = 1; i < args.size(); ++i)
    {
        string arg_key = args[i];
        string arg_val = i + 1 < args.size() ? args[i + 1] : "";

        if (arg_key == "--help")
        {
            printHelp();
            return 1;
        }
        else if (arg_key == "-i" || arg_key == "--input")
        {
            input_path = arg_val;
            i++;
        }
        else if (arg_key == "-o" || arg_key == "--output")
        {
            output_path = arg_val;
            i++;
        }
        else if (arg_key == "-m" || arg_key == "--manifest")
        {
            manifest_path = arg_val;
            i++;
        }
        else
            cout << "[WARNING] Found unknown argument: " << arg_key << endl;
    }

    return 0;
}

int check_pathes(path &input_path, path &output_path, path &manifest_path)
{
    if (!exists(input_path))
    {
        cout << "Input path does not exist" << endl;
        return 1;
    }
    else if (!is_directory(input_path))
    {
        cout << "Input path must be a directory" << endl;
        return 1;
    }
    else if (exists(output_path))
    {
        cout << "Output path already exists" << endl;
        return 1;
    }
    else if (is_directory(output_path))
    {
        cout << "Output path must be a file" << endl;
        return 1;
    }
    else if (!exists(manifest_path))
    {
        cout << "Manifest path does not exist" << endl;
        return 1;
    }
    else if (is_directory(manifest_path))
    {
        cout << "Manifest path must be a file" << endl;
        return 1;
    }
    return 0;
}

void list_items(const path &dir_path, vector<path> &items)
{
    if (!exists(dir_path) || !is_directory(dir_path))
    {
        return;
    }

    for (const auto &entry : directory_iterator(dir_path))
    {
        const path &p = entry.path();

        if (is_directory(p))
        {
            list_items(p, items);
            items.push_back(p);
        }
        else if (is_regular_file(p))
        {
            items.push_back(p);
        }
    }
}

bool in_array(const string &value, const vector<string> &array)
{
    return std::find(array.begin(), array.end(), value) != array.end();
}