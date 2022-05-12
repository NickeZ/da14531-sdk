use std::env;
use std::path::PathBuf;

use lazy_static::lazy_static;

lazy_static! {
    static ref INCLUDE_PATHS: Vec<&'static str> = vec![
        "/sdk/app_modules/api",
        "/sdk/ble_stack/controller/em",
        "/sdk/ble_stack/controller/llc",
        "/sdk/ble_stack/controller/lld",
        "/sdk/ble_stack/controller/llm",
        "/sdk/ble_stack/ea/api",
        "/sdk/ble_stack/em/api",
        "/sdk/ble_stack/hci/api",
        "/sdk/ble_stack/hci/src",
        "/sdk/ble_stack/host/att",
        "/sdk/ble_stack/host/att/attc",
        "/sdk/ble_stack/host/att/attm",
        "/sdk/ble_stack/host/att/atts",
        "/sdk/ble_stack/host/gap",
        "/sdk/ble_stack/host/gap/gapc",
        "/sdk/ble_stack/host/gap/gapm",
        "/sdk/ble_stack/host/gatt",
        "/sdk/ble_stack/host/gatt/gattc",
        "/sdk/ble_stack/host/gatt/gattm",
        "/sdk/ble_stack/host/l2c/l2cc",
        "/sdk/ble_stack/host/l2c/l2cm",
        "/sdk/ble_stack/host/smp",
        "/sdk/ble_stack/host/smp/smpc",
        "/sdk/ble_stack/host/smp/smpm",
        "/sdk/ble_stack/profiles",
        "/sdk/ble_stack/profiles/custom",
        "/sdk/ble_stack/profiles/custom/custs/api",
        "/sdk/ble_stack/profiles/dis/diss/api",
        "/sdk/ble_stack/rwble_hl",
        "/sdk/ble_stack/rwble",
        "/sdk/common_project_files",
        "/sdk/platform/arch",
        "/sdk/platform/arch/boot",
        "/sdk/platform/arch/boot/GCC",
        "/sdk/platform/arch/compiler",
        "/sdk/platform/arch/compiler/GCC",
        "/sdk/platform/arch/ll",
        "/sdk/platform/arch/main",
        "/sdk/platform/core_modules/arch_console",
        "/sdk/platform/core_modules/common/api",
        "/sdk/platform/core_modules/crypto",
        "/sdk/platform/core_modules/dbg/api",
        "/sdk/platform/core_modules/gtl/api",
        "/sdk/platform/core_modules/gtl/src",
        "/sdk/platform/core_modules/h4tl/api",
        "/sdk/platform/core_modules/ke/api",
        "/sdk/platform/core_modules/ke/src",
        "/sdk/platform/core_modules/nvds/api",
        "/sdk/platform/core_modules/rf/api",
        "/sdk/platform/core_modules/rwip/api",
        "/sdk/platform/core_modules/rwip/api",
        "/sdk/platform/driver/adc",
        "/sdk/platform/driver/ble",
        "/sdk/platform/driver/dma",
        "/sdk/platform/driver/gpio",
        "/sdk/platform/driver/hw_otpc",
        "/sdk/platform/driver/i2c_eeprom",
        "/sdk/platform/driver/i2c",
        "/sdk/platform/driver/reg",
        "/sdk/platform/driver/spi_flash",
        "/sdk/platform/driver/spi",
        "/sdk/platform/driver/syscntl",
        "/sdk/platform/driver/trng",
        "/sdk/platform/driver/uart",
        "/sdk/platform/include",
        "/sdk/platform/include/CMSIS/5.6.0/Include",
        "/sdk/platform/system_library/include",
        "/sdk/platform/utilities/otp_cs",
        "/sdk/platform/utilities/otp_hdr",
        "/third_party/hash",
        "/third_party/irng",
    ];

    static ref CONFIG_HEADERS: Vec<&'static str> = vec![
        "da1458x_config_basic.h",
        "da1458x_config_advanced.h",
        "user_config.h"
    ];

    static ref SDK_SOURCES: Vec<&'static str> = vec![
        "/sdk/app_modules/src/app_common/app_msg_utils.c",
        "/sdk/app_modules/src/app_common/app_task.c",
        // "/sdk/app_modules/src/app_common/app.c",
        "/sdk/app_modules/src/app_custs/app_customs_task.c",
        // "/sdk/app_modules/src/app_custs/app_customs.c",
        "/sdk/app_modules/src/app_default_hnd/app_default_handlers.c",
        "/sdk/app_modules/src/app_diss/app_diss_task.c",
        "/sdk/app_modules/src/app_diss/app_diss.c",
        "/sdk/app_modules/src/app_entry/app_entry_point.c",
        "/sdk/ble_stack/profiles/custom/custs/src/custs1_task.c",
        // "/sdk/ble_stack/profiles/custom/custs/src/custs1.c",
        "/sdk/ble_stack/profiles/prf.c",
        "/sdk/ble_stack/rwble/rwble.c",
        "/sdk/platform/arch/boot/system_DA14531.c",
        "/sdk/platform/arch/main/arch_main.c",
        "/sdk/platform/arch/main/arch_rom.c",
        "/sdk/platform/arch/main/arch_rom.c",
        "/sdk/platform/arch/main/arch_sleep.c",
        "/sdk/platform/arch/main/arch_system.c",
        "/sdk/platform/arch/main/hardfault_handler.c",
        "/sdk/platform/arch/main/jump_table.c",
        "/sdk/platform/arch/main/nmi_handler.c",
        "/sdk/platform/core_modules/crypto/aes.c",
        "/sdk/platform/core_modules/crypto/aes_api.c",
        "/sdk/platform/core_modules/crypto/aes_cbc.c",
        "/sdk/platform/core_modules/crypto/sw_aes.c",
        "/sdk/platform/core_modules/crypto/aes_task.c",
        "/sdk/platform/core_modules/nvds/src/nvds.c",
        "/sdk/platform/core_modules/rf/src/ble_arp.c",
        "/sdk/platform/core_modules/rf/src/rf_531.c",
        "/sdk/platform/core_modules/rwip/src/rwip.c",
        "/sdk/platform/driver/adc/adc_531.c",
        "/sdk/platform/driver/gpio/gpio.c",
        "/sdk/platform/driver/hw_otpc/hw_otpc_531.c",
        "/sdk/platform/driver/syscntl/syscntl.c",
        "/sdk/platform/driver/trng/trng.c",
        "/sdk/platform/driver/uart/uart_utils.c",
        "/sdk/platform/driver/uart/uart.c",
        "/sdk/platform/utilities/otp_cs/otp_cs.c",
        "/sdk/platform/utilities/otp_hdr/otp_hdr.c"
    ];


    static ref SDK_SOURCES_ASM: Vec<&'static str> = vec![
    ];


    static ref C_PROJ_SOURCES: Vec<&'static str> = vec![
        // "src/ble_stack/profiles/custom/custs/custs1.c",
    ];

}

fn generate_user_modules_config() {
    let exclude_dlg_custs1 = if cfg!(feature = "profile_custom_server1") {
        0
    } else {
        1
    };
    let exclude_dlg_custs2 = if cfg!(feature = "profile_custom_server2") {
        0
    } else {
        1
    };

    let exclude_dlg_diss = if cfg!(feature = "profile_dis_server") {
        0
    } else {
        1
    };

    let exclude_dlg_proxr = if cfg!(feature = "profile_prox_reporter") {
        0
    } else {
        1
    };

    let exclude_dlg_bass = if cfg!(feature = "profile_batt_server") {
        0
    } else {
        1
    };

    let exclude_dlg_suotar = if cfg!(feature = "profile_suota_receiver") {
        0
    } else {
        1
    };

    let exclude_dlg_findt = if cfg!(feature = "profile_findme_target") {
        0
    } else {
        1
    };

    let exclude_dlg_findl = if cfg!(feature = "profile_findme_locator") {
        0
    } else {
        1
    };

    let header = format!(
        "
#ifndef _USER_MODULES_CONFIG_H_
#define _USER_MODULES_CONFIG_H_

#define EXCLUDE_DLG_GAP             (0)
#define EXCLUDE_DLG_TIMER           (0)
#define EXCLUDE_DLG_MSG             (1)
#define EXCLUDE_DLG_SEC             (1)
#define EXCLUDE_DLG_DISS            ({exclude_dlg_diss})
#define EXCLUDE_DLG_PROXR           ({exclude_dlg_proxr})
#define EXCLUDE_DLG_BASS            ({exclude_dlg_bass})
#define EXCLUDE_DLG_FINDL           ({exclude_dlg_findl})
#define EXCLUDE_DLG_FINDT           ({exclude_dlg_findt})
#define EXCLUDE_DLG_SUOTAR          ({exclude_dlg_suotar})
#define EXCLUDE_DLG_CUSTS1          ({exclude_dlg_custs1})
#define EXCLUDE_DLG_CUSTS2          ({exclude_dlg_custs2})

#endif // _USER_MODULES_CONFIG_H_
    "
    );

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    std::fs::write(out_path.join("user_modules_config.h"), header).unwrap();
}

fn generate_user_callback_config() {
    let app_process_catch_rest_cb = if cfg!(feature = "custom_rest_evt_cb") {
        "
extern void __catch_rest_hndl(ke_msg_id_t const msgid,
                              void const *param,
                              ke_task_id_t const dest_id,
                              ke_task_id_t const src_id);
#define app_process_catch_rest_cb       __catch_rest_hndl"
    } else {
        "static const catch_rest_event_func_t NULL;"
    };

    let header = format!(
        "
#ifndef _USER_CALLBACK_CONFIG_H_
#define _USER_CALLBACK_CONFIG_H_


#include <stdio.h>
#include \"app_callback.h\"
#include \"app_default_handlers.h\"
#include \"app_entry_point.h\"
#include \"app_prf_types.h\"
#include \"user_peripheral.h\"


extern const struct app_callbacks user_app_callbacks;
extern const struct default_app_operations user_default_app_operations;
extern const struct arch_main_loop_callbacks user_app_main_loop_callbacks;

{app_process_catch_rest_cb}

#endif // _USER_CALLBACK_CONFIG_H_"
    );

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    std::fs::write(out_path.join("user_callback_config.h"), header).unwrap();
}

fn setup_build() -> (
    Vec<String>,
    Vec<String>,
    Vec<String>,
    Vec<String>,
    Vec<(String, Option<String>)>,
) {
    let sdk_path = env::var("SDK_PATH").expect("SDK_PATH not set!");
    let config_path = env::var("CONFIG_PATH").expect("CONFIG_PATH not set!");

    #[allow(unused_mut)]
    let mut include_dirs = INCLUDE_PATHS.clone();

    #[allow(unused_mut)]
    let mut sdk_sources: Vec<_> = SDK_SOURCES.clone();

    #[allow(unused_mut)]
    let mut sdk_sources_asm: Vec<_> = SDK_SOURCES_ASM.clone();

    let mut defines = vec![("__DA14531__", None)];

    #[allow(unused_mut)]
    let mut include_files: Vec<&str> = Vec::new();

    #[cfg(feature = "address_mode_public")]
    {
        defines.push(("USER_CFG_ADDRESS_MODE", Some("APP_CFG_ADDR_PUB")));
    }

    #[cfg(feature = "address_mode_static")]
    {
        defines.push(("USER_CFG_ADDRESS_MODE", Some("APP_CFG_ADDR_STATIC")));
    }

    #[cfg(feature = "ble_server_profiles")]
    {
        defines.push(("BLE_SERVER_PRF", None));
    }

    #[cfg(feature = "ble_client_profiles")]
    {
        defines.push(("BLE_CLIENT_PRF", None));
    }

    #[cfg(feature = "profile_gatt_client")]
    {
        defines.push(("CFG_PRF_GATTC", None));
        include_dirs.push("/sdk/ble_stack/profiles/gatt/gatt_client/api");
        sdk_sources.push("/sdk/app_modules/src/app_gattc/app_gattc.c");
    }

    #[cfg(feature = "profile_prox_reporter")]
    {
        defines.push(("CFG_PRF_PXPR", None));
        include_dirs.push("/sdk/ble_stack/profiles/prox/proxr/api");
        include_files.push("/sdk/app_modules/api/app_proxr.h");
    }

    #[cfg(feature = "profile_batt_server")]
    {
        defines.push(("CFG_PRF_PXPR", None));
        include_dirs.push("/sdk/ble_stack/profiles/prox/proxr/api");
        include_files.push("/sdk/app_modules/api/app_proxr.h");
    }

    #[cfg(feature = "profile_findme_target")]
    {
        defines.push(("CFG_PRF_FMPT", None));
        include_dirs.push("/sdk/ble_stack/profiles/find/findt/api");
        include_dirs.push("/sdk/ble_stack/profiles/find");
        include_files.push("/sdk/app_modules/api/app_findme.h");
    }

    let mut include_dirs: Vec<_> = include_dirs
        .iter()
        .map(|path| format!("{}{}", sdk_path, path))
        .collect();

    include_dirs.push("./include".to_string());
    include_dirs.push(format!("{}/..", config_path));
    include_dirs.push(env::var("OUT_DIR").unwrap());

    include_dirs.push(config_path);

    let mut include_files: Vec<_> = include_files
        .iter()
        .map(|path| format!("{}{}", sdk_path, path))
        .collect();

    let mut config_headers: Vec<_> = CONFIG_HEADERS.iter().map(|s| s.to_string()).collect();
    include_files.append(&mut config_headers);

    let sdk_sources: Vec<_> = sdk_sources
        .iter()
        .map(|path| format!("{}{}", sdk_path, path))
        .chain(C_PROJ_SOURCES.iter().map(|s| s.to_string()))
        .collect();

    let sdk_sources_asm: Vec<_> = sdk_sources_asm
        .iter()
        .map(|path| format!("{}{}", sdk_path, path))
        .collect();

    let defines: Vec<_> = defines
        .iter()
        .map(|(key, value)| (key.to_string(), value.map(|value| value.to_string())))
        .collect();

    (
        include_dirs,
        include_files,
        sdk_sources,
        sdk_sources_asm,
        defines,
    )
}

fn generate_bindings(
    include_dirs: &Vec<String>,
    include_files: &Vec<String>,
    defines: &Vec<(String, Option<String>)>,
    rustify_enums: &Vec<&str>,
) {
    let mut builder = bindgen::Builder::default()
        .header("bindings.h")
        .ctypes_prefix("cty")
        .use_core()
        .size_t_is_usize(true)
        .clang_arg("-DUSER_DEVICE_NAME_LEN=0")
        .clang_arg("-I/usr/lib/newlib-nano/arm-none-eabi/include")
        .clang_arg("-Wno-expansion-to-defined");

    for (key, value) in defines {
        if let Some(value) = value {
            builder = builder.clang_arg(format!("-D{}={}", key, value));
        } else {
            builder = builder.clang_arg(format!("-D{}", key));
        }
    }

    for inc_dir in include_dirs {
        builder = builder.clang_arg(format!("-I{}", inc_dir));
    }

    for inc_file in include_files {
        builder = builder.clang_args(vec!["-include", inc_file]);
    }

    for re in rustify_enums {
        builder = builder.rustified_enum(re)
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn compile_sdk(
    include_dirs: &Vec<String>,
    include_files: &Vec<String>,
    defines: &Vec<(String, Option<String>)>,
    sdk_sources: &Vec<String>,
    _sdk_sources_asm: &Vec<String>,
) {
    let mut cc_builder = cc::Build::new();

    let mut cc_builder = cc_builder
        .debug(true)
        .target("thumbv6m-none-eabi")
        .flag("-march=armv6-m")
        .flag("-Wno-expansion-to-defined")
        .flag("-Wno-unused-parameter")
        .define("USER_DEVICE_NAME_LEN", Some("0"));

    for inc_dir in include_dirs {
        cc_builder = cc_builder.include(inc_dir);
    }

    for inc_file in include_files {
        cc_builder = cc_builder.flag(&format!("-include{}", inc_file));
    }

    for (key, value) in defines {
        cc_builder = cc_builder.define(key, value.as_ref().map(|v| v.as_str()));
    }

    for file in sdk_sources {
        cc_builder = cc_builder.file(file);
    }

    cc_builder.compile("libsdk.a");
}

fn main() {
    let (include_dirs, include_files, sdk_sources, sdk_sources_asm, defines) = setup_build();

    generate_user_callback_config();
    generate_user_modules_config();

    generate_bindings(
        &include_dirs,
        &include_files,
        &defines,
        &vec![
            "hl_err",
            "ke_msg_status_tag",
            "process_event_response",
            "syscntl_dcdc_level_t",
            "APP_MSG",
        ],
    );

    compile_sdk(
        &include_dirs,
        &include_files,
        &defines,
        &sdk_sources,
        &sdk_sources_asm,
    );

    println!("cargo:rerun-if-changed=bindings.h");
    println!("cargo:rerun-if-changed=build.rs");

    let config_path = env::var("CONFIG_PATH").expect("CONFIG_PATH not set!");
    println!("cargo:rerun-if-changed=include/da1458x_config_basic.h");
    println!("cargo:rerun-if-changed=include/da1458x_config_advanced.h",);
    println!("cargo:rerun-if-changed=include/user_periph_setup.h",);
    println!("cargo:rerun-if-changed={}/user_config.h", config_path);
    println!(
        "cargo:rerun-if-changed={}/user_callback_config.h",
        config_path
    );
    println!(
        "cargo:rerun-if-changed={}/user_profiles_config.h",
        config_path
    );

    for c_proj_source in C_PROJ_SOURCES.iter() {
        println!("cargo:rerun-if-changed={}", c_proj_source);
    }
}
