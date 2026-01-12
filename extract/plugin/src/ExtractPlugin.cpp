// TODO: aviutl2-rsで書き換える
#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <stdint.h>
#include <string>
#include <thread>
#include <windows.h>

#include <aviutl2_sdk/plugin2.h>
#include <winnt.h>

EDIT_HANDLE *edit_handle = nullptr;

#define fn auto
#define let auto
#define EXPORT __declspec(dllexport)
#define c_fn extern "C" auto

fn dll_path() -> std::wstring {
  HMODULE hModule = nullptr;
  GetModuleHandleExW(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS |
                         GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
                     reinterpret_cast<LPCWSTR>(&dll_path), &hModule);
  wchar_t path[MAX_PATH];
  GetModuleFileNameW(hModule, path, MAX_PATH);
  return std::wstring(path);
}

fn plugin_data_root() -> std::wstring {
  return dll_path() + L"/../../extract/";
}
fn effect_data_root() -> std::wstring {
  return plugin_data_root() + L"effects/";
}

struct EnumerateEffectsState {
  int effect_count;
};
struct ExtractEffectsState {
  std::wstring effect_name;
  int effect_count;
};

fn collect_effects() -> void {
  EnumerateEffectsState state = {0};
  std::filesystem::create_directories(effect_data_root());
  edit_handle->enum_effect_name(
      &state, [](void *param, LPCWSTR name, int type, int flag) -> void {
        let state = static_cast<EnumerateEffectsState *>(param);
        state->effect_count += 1;

        let extract_state =
            ExtractEffectsState{.effect_name = std::wstring(name),
                                .effect_count = state->effect_count};
        edit_handle->call_edit_section_param(
            &extract_state, [](void *param, EDIT_SECTION *edit) -> void {
              let state = static_cast<ExtractEffectsState *>(param);
              let oh = edit->create_object(state->effect_name.c_str(), 0, 0, 1);
              if (oh != nullptr) {
                let alias = edit->get_object_alias(oh);
                if (alias != nullptr) {
                  let file_path = effect_data_root() +
                                  std::to_wstring(state->effect_count) +
                                  L".object";
                  std::ofstream output_file(file_path, std::ios::binary);
                  output_file.write(alias, std::strlen(alias));
                  output_file.close();
                }
                edit->delete_object(oh);
              }
            });
      });
}

fn mode_path() -> std::wstring { return plugin_data_root() + L"mode.txt"; }

fn write_shutdown() -> void {
  std::ofstream mode_file_out(mode_path(), std::ios::trunc);
  mode_file_out << "shutdown";
  mode_file_out.close();
}

fn defer_collect_effects() -> void {
  std::thread([]() {
    collect_effects();
    write_shutdown();
    edit_handle->restart_host_app();
  }).detach();
}

c_fn on_project_load(PROJECT_FILE *_project_file) -> void {
  std::ifstream mode_file(mode_path());
  std::string mode;
  std::getline(mode_file, mode);
  mode_file.close();
  if (mode == "extract") {
    defer_collect_effects();
  } else if (mode == "reboot") {
    write_shutdown();
    edit_handle->restart_host_app();
  } else if (mode == "shutdown") {
    std::filesystem::remove(mode_path());
    std::exit(0);
  } else {
    throw std::runtime_error("invalid mode");
  }
}

c_fn EXPORT InitializePlugin(DWORD version) -> bool {
  let mode_file_path = mode_path();
  if (!std::filesystem::exists(mode_file_path)) {
    return false;
  }
  return true;
}

c_fn EXPORT RegisterPlugin(HOST_APP_TABLE *host) -> void {
  host->set_plugin_information(L"Reboot Plugin");
  edit_handle = host->create_edit_handle();
  host->register_project_load_handler(on_project_load);
}
