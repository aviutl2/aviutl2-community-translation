#include <cstdlib>
#include <filesystem>
#include <stdint.h>
#include <string>
#include <fstream>
#include <windows.h>

#include <aviutl2_sdk/plugin2.h>

EDIT_HANDLE *edit_handle = nullptr;

#define fn auto
#define let auto
#define EXPORT __declspec(dllexport)
#define c_fn extern "C" auto

fn executable_path() -> std::wstring {
  wchar_t path[MAX_PATH];
  GetModuleFileNameW(NULL, path, MAX_PATH);
  return std::wstring(path);
}

c_fn on_project_load(PROJECT_FILE *_project_file) -> void {
  let flag_path = executable_path() + L"/../data/reboot.flag";
  if (std::filesystem::exists(flag_path)) {
    std::exit(0);
  } else {
    std::ofstream flag_file(flag_path);
    flag_file << "Reboot!";
    flag_file.close();
    edit_handle->restart_host_app();
  }
}

c_fn EXPORT RegisterPlugin(HOST_APP_TABLE *host) -> void {
  host->set_plugin_information(L"Reboot Plugin");
  edit_handle = host->create_edit_handle();
  host->register_project_load_handler(on_project_load);
}
