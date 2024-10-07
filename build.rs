use std::convert::AsRef;
use std::env;
use std::ffi::OsStr;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{Read, Write};
use std::num::ParseIntError;
use std::str::FromStr;

use std::path::{Path, PathBuf};

const INCLUDE_PATHS: &[&str] = &[
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
    "/sdk/ble_stack/rwble",
    "/sdk/ble_stack/rwble_hl",
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
    "/sdk/platform/driver/i2c",
    "/sdk/platform/driver/i2c_eeprom",
    "/sdk/platform/driver/reg",
    "/sdk/platform/driver/spi",
    "/sdk/platform/driver/spi_flash",
    "/sdk/platform/driver/syscntl",
    "/sdk/platform/driver/trng",
    "/sdk/platform/driver/uart",
    "/sdk/platform/include",
    "/sdk/platform/include/CMSIS/5.9.0/CMSIS/Core/Include",
    "/sdk/platform/system_library/include",
    "/sdk/platform/utilities/otp_cs",
    "/sdk/platform/utilities/otp_hdr",
    "/third_party/hash",
    "/third_party/irng",
    "/third_party/rand",
];
const CONFIG_HEADERS: &[&str] = &[
    "da1458x_config_basic.h",
    "da1458x_config_advanced.h",
    "user_config.h",
];
const SDK_C_SOURCES: &[&str] = &[
    "/sdk/app_modules/src/app_common/app_msg_utils.c",
    "/sdk/app_modules/src/app_common/app_task.c",
    "/sdk/app_modules/src/app_custs/app_customs_task.c",
    "/sdk/app_modules/src/app_default_hnd/app_default_handlers.c",
    "/sdk/app_modules/src/app_entry/app_entry_point.c",
    "/sdk/ble_stack/profiles/custom/custs/src/custs1_task.c",
    "/sdk/ble_stack/profiles/prf.c",
    "/sdk/ble_stack/rwble/rwble.c",
    "/sdk/platform/arch/boot/system_DA14531.c",
    "/sdk/platform/arch/main/arch_main.c",
    "/sdk/platform/arch/main/arch_rom.c",
    "/sdk/platform/arch/main/arch_sleep.c",
    "/sdk/platform/arch/main/arch_system.c",
    "/sdk/platform/arch/main/hardfault_handler.c",
    "/sdk/platform/arch/main/jump_table.c",
    "/sdk/platform/arch/main/nmi_handler.c",
    "/sdk/platform/core_modules/crypto/aes.c",
    "/sdk/platform/core_modules/crypto/aes_api.c",
    "/sdk/platform/core_modules/crypto/aes_cbc.c",
    "/sdk/platform/core_modules/crypto/aes_task.c",
    "/sdk/platform/core_modules/crypto/sw_aes.c",
    "/sdk/platform/core_modules/nvds/src/nvds.c",
    "/sdk/platform/core_modules/rf/src/ble_arp.c",
    "/sdk/platform/core_modules/rf/src/rf_531.c",
    "/sdk/platform/core_modules/rwip/src/rwip.c",
    "/sdk/platform/driver/adc/adc_531.c",
    "/sdk/platform/driver/gpio/gpio.c",
    "/sdk/platform/driver/hw_otpc/hw_otpc_531.c",
    "/sdk/platform/driver/spi/spi_531.c",
    "/sdk/platform/driver/spi_flash/spi_flash.c",
    "/sdk/platform/driver/syscntl/syscntl.c",
    "/sdk/platform/driver/trng/trng.c",
    "/sdk/platform/system_library/src/DA14531/system_library_531.c",
    "/sdk/platform/utilities/otp_cs/otp_cs.c",
    "/sdk/platform/utilities/otp_hdr/otp_hdr.c",
    "/third_party/rand/chacha20.c",
];
const SDK_ASM_SOURCES: &[&str] = &[
    "/sdk/platform/arch/boot/GCC/ivtable_DA14531.S",
    "/sdk/platform/arch/boot/GCC/startup_DA14531.S",
];

enum ConfigItemValue {
    Undefined,
    Defined,
    Number(i32),
    Raw(String),
}

impl FromStr for ConfigItemValue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfigItemValue::*;
        if s.to_lowercase() == "undefined" {
            Ok(Undefined)
        } else if s.to_lowercase() == "defined" {
            Ok(Defined)
        } else if let Ok(integer) = s.parse() {
            Ok(Number(integer))
        } else {
            Ok(Raw(format!("\"{s}\"")))
        }
    }
}

struct ConfigItem {
    name: String,
    value: ConfigItemValue,
}

impl ConfigItem {
    fn new(name: &str, mut value: ConfigItemValue) -> Self {
        if let Ok(var) = env::var(format!("DA14531_{}", name)) {
            // Unwrap is fine because implementation never errors.
            // See `impl FromStr for ConfigItemValue`.
            value = var.parse().unwrap()
        }
        ConfigItem {
            name: name.to_string(),
            value,
        }
    }
}

impl Display for ConfigItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match &self.value {
            ConfigItemValue::Undefined => write!(f, "#undef {}", self.name),
            ConfigItemValue::Defined => write!(f, "#define {}", self.name),
            ConfigItemValue::Number(n) => write!(f, "#define {} ({})", self.name, n),
            ConfigItemValue::Raw(s) => write!(f, "#define {} {}", self.name, s),
        }
    }
}

fn read_file(file: impl AsRef<Path>) -> String {
    let mut fh = File::open(file).expect("File should be openable");
    let mut res = String::new();
    fh.read_to_string(&mut res)
        .expect("File should be readable");
    res
}

fn getenv(var: impl AsRef<OsStr>) -> String {
    env::var(var).unwrap()
}

fn generate_header(name: &str, vars: &[ConfigItem]) {
    let template = format!("{}.in", name);
    let template = PathBuf::from_iter(["include", &template]);
    let template = read_file(template);
    let res = envsubst::substitute(
        template,
        &vars
            .iter()
            .map(|v| (v.name.to_string(), v.to_string()))
            .collect(),
    )
    .unwrap();
    let mut outfile = File::create(PathBuf::from_iter([&getenv("OUT_DIR"), name])).unwrap();
    outfile.write_all(res.as_bytes()).unwrap();
}

// All default values can be overridden using environment variables, the prefix is "DA14531_". So
// for example DA14531_CFG_TRNG=Undefined will set CFG_TRNG to Undefined.

fn generate_da14531_config_advanced() {
    use ConfigItemValue::*;
    #[rustfmt::skip]
    let vars = &[
        ConfigItem::new("CFG_TRNG", Defined),
        ConfigItem::new("CFG_USE_CHACHA20_RAND", Undefined),
        ConfigItem::new("DB_HEAP_SZ", Undefined),
        ConfigItem::new("ENV_HEAP_SZ", Undefined),
        ConfigItem::new("MSG_HEAP_SZ", Undefined),
        ConfigItem::new("NON_RET_HEAP_SZ", Undefined),
        ConfigItem::new("CFG_USE_AES", Defined),
        ConfigItem::new("CFG_AES_DECRYPT", Defined),
    ];
    generate_header("da14531_config_advanced.h", vars);
}

fn generate_da14531_config_basic() {
    use ConfigItemValue::*;
    #[rustfmt::skip]
    let vars = &[
        ConfigItem::new("CFG_APP", Defined),
        ConfigItem::new("CFG_APP_SECURITY",  if cfg!(feature = "app_security") { Defined } else { Undefined }),
        ConfigItem::new("CFG_WDOG", Defined),
        ConfigItem::new("CFG_WDG_TRIGGER_HW_RESET_IN_PRODUCTION_MODE", Undefined),
        ConfigItem::new("CFG_MAX_CONNECTIONS", Number(1)),
        ConfigItem::new("CFG_DEVELOPMENT_DEBUG", Defined),
        ConfigItem::new("CFG_PRINTF", Undefined),
        ConfigItem::new("CFG_UART1_SDK", Undefined),
        ConfigItem::new("CFG_SPI_FLASH_ENABLE", Undefined),
        ConfigItem::new("CFG_I2C_EEPROM_ENABLE", Undefined),
        ConfigItem::new("CFG_UART_DMA_SUPPORT", Undefined),
        ConfigItem::new("CFG_SPI_DMA_SUPPORT", Undefined),
        ConfigItem::new("CFG_I2C_DMA_SUPPORT", Undefined),
        ConfigItem::new("CFG_ADC_DMA_SUPPORT", Undefined),
        ConfigItem::new("CFG_POWER_MODE_BYPASS", Undefined),
    ];
    generate_header("da14531_config_basic.h", vars);
}

fn generate_user_modules_config() {
    use ConfigItemValue::*;
    #[rustfmt::skip]
    let vars = &[
        ConfigItem::new("EXCLUDE_DLG_GAP", Number(0)),
        ConfigItem::new("EXCLUDE_DLG_TIMER", Number(0)),
        ConfigItem::new("EXCLUDE_DLG_MSG", Number(1)),
        ConfigItem::new("EXCLUDE_DLG_SEC", Number(1)),
        ConfigItem::new("EXCLUDE_DLG_DISS", Number(if cfg!(feature = "profile_dis_server") { 0 } else { 1 })),
        ConfigItem::new("EXCLUDE_DLG_PROXR", Number(if cfg!(feature = "profile_prox_reporter") { 0 } else { 1 })),
        ConfigItem::new("EXCLUDE_DLG_BASS", Number(if cfg!(feature = "profile_batt_server") { 0 } else { 1 })),
        ConfigItem::new("EXCLUDE_DLG_FINDL", Number(if cfg!(feature = "profile_findme_locator") { 0 } else { 1 })),
        ConfigItem::new("EXCLUDE_DLG_FINDT", Number(if cfg!(feature = "profile_findme_target") { 0 } else { 1 })),
        ConfigItem::new("EXCLUDE_DLG_SUOTAR", Number(if cfg!(feature = "profile_suota_receiver") { 0 } else { 1 })),
        ConfigItem::new("EXCLUDE_DLG_CUSTS1", Number(if cfg!(feature = "profile_custom_server1") { 0 } else { 1 })),
        ConfigItem::new("EXCLUDE_DLG_CUSTS2", Number(if cfg!(feature = "profile_custom_server2") { 0 } else { 1 })),
    ];
    generate_header("user_modules_config.h", vars);
}

fn generate_user_profiles_config() {
    use ConfigItemValue::*;
    #[rustfmt::skip]
    let vars = &[
        ConfigItem::new("CFG_PRF_CUST1", if cfg!(feature = "profile_custom_server1") { Defined } else { Undefined }),
        ConfigItem::new("CFG_PRF_CUST2", if cfg!(feature = "profile_custom_server2") { Defined } else { Undefined }),
        ConfigItem::new("CFG_PRF_DISS", if cfg!(feature = "profile_dis_server") { Defined } else { Undefined }),
        ConfigItem::new("CFG_PRF_PXPR", if cfg!(feature = "profile_prox_reporter") { Defined } else { Undefined }),
        ConfigItem::new("CFG_PRF_BASS", if cfg!(feature = "profile_batt_server") { Defined } else { Undefined }),
        ConfigItem::new("CFG_PRF_SUOTAR", if cfg!(feature = "profile_suota_receiver") { Defined } else { Undefined }),
        ConfigItem::new("CFG_PRF_FMPT", if cfg!(feature = "profile_findme_target") { Defined } else { Undefined }),
        ConfigItem::new("CFG_PRF_FMPL", if cfg!(feature = "profile_findme_locator") { Defined } else { Undefined }),
        ConfigItem::new("CFG_PRF_GATTC", if cfg!(feature = "profile_gatt_client") { Defined } else { Undefined }),
    ];
    generate_header("user_profiles_config.h", vars);
}

fn generate_user_config() {
    #[cfg(all(feature = "address_mode_public", feature = "address_mode_static"))]
    compile_error!("Only one address mode feature flag can be set!");

    let address_mode = if cfg!(feature = "address_mode_public") {
        "APP_CFG_ADDR_PUB"
    } else if cfg!(feature = "address_mode_static") {
        "APP_CFG_ADDR_STATIC"
    } else {
        panic!("One address mode feature flag has to be set!");
    };

    #[cfg(any(
        all(feature = "sleep_mode_off", feature = "sleep_mode_ext_on"),
        all(feature = "sleep_mode_off", feature = "sleep_mode_ext_otp_copy_on"),
        all(feature = "sleep_mode_ext_on", feature = "sleep_mode_ext_otp_copy_on")
    ))]
    compile_error!("Only one sleep mode feature flag can be set!");

    let sleep_mode = if cfg!(feature = "sleep_mode_off") {
        "ARCH_SLEEP_OFF"
    } else if cfg!(feature = "sleep_mode_ext_on") {
        "ARCH_EXT_SLEEP_ON"
    } else if cfg!(feature = "sleep_mode_ext_otp_copy_on") {
        "ARCH_EXT_SLEEP_OTP_COPY_ON"
    } else {
        panic!("One sleep mode feature flag has to be set!");
    };

    #[rustfmt::skip]
    let vars = &[
        ConfigItem::new("USER_CFG_ADDRESS_MODE", ConfigItemValue::Raw(address_mode.to_string())),
        ConfigItem::new("USER_CFG_CNTL_PRIV_MODE", ConfigItemValue::Raw("APP_CFG_CNTL_PRIV_MODE_NETWORK".to_string())),
        ConfigItem::new("USER_CFG_ARCH_SLEEP_MODE", ConfigItemValue::Raw(sleep_mode.to_string())),
        ConfigItem::new("USER_DEVICE_NAME", ConfigItemValue::Raw("\"\"".to_string())),
    ];
    generate_header("user_config.h", vars);
}

fn setup_build() -> (
    Vec<String>,
    Vec<String>,
    Vec<String>,
    Vec<String>,
    Vec<(String, Option<String>)>,
) {
    let sdk_path = env::var("SDK_PATH")
        .map(|path| Path::new(&path).to_path_buf())
        .unwrap_or(PathBuf::new().join("..").join("sdk"))
        .to_str()
        .unwrap()
        .to_string();

    #[allow(unused_mut)]
    let mut include_dirs: Vec<String> = INCLUDE_PATHS.iter().map(|s| String::from(*s)).collect();

    #[allow(unused_mut)]
    let mut sdk_c_sources: Vec<String> = SDK_C_SOURCES.iter().map(|s| String::from(*s)).collect();

    let defines: Vec<(&str, Option<&str>)> = vec![("__DA14531__", None)];

    #[allow(unused_mut)]
    let mut include_files: Vec<String> = Vec::new();

    #[cfg(feature = "profile_gatt_client")]
    {
        include_dirs.push(translate_path(
            "/sdk/ble_stack/profiles/gatt/gatt_client/api",
        ));
        sdk_c_sources.push(translate_path("/sdk/app_modules/src/app_gattc/app_gattc.c"));
    }

    #[cfg(feature = "profile_prox_reporter")]
    {
        include_dirs.push(translate_path("/sdk/ble_stack/profiles/prox/proxr/api"));
        include_files.push(translate_path("/sdk/app_modules/api/app_proxr.h"));
    }

    #[cfg(feature = "profile_batt_server")]
    {
        include_dirs.push(translate_path("/sdk/ble_stack/profiles/bas/bass/api"));
        include_files.push(translate_path("/sdk/app_modules/api/app_bass.h"));
    }

    #[cfg(feature = "profile_findme_target")]
    {
        include_dirs.push(translate_path("/sdk/ble_stack/profiles/find/findt/api"));
        include_dirs.push(translate_path("/sdk/ble_stack/profiles/find"));
        include_files.push(translate_path("/sdk/app_modules/api/app_findme.h"));
    }

    let mut include_dirs: Vec<_> = include_dirs
        .iter()
        .map(|path| format!("{}{}", sdk_path, path))
        .collect();

    include_dirs.push(
        env::current_dir()
            .unwrap()
            .to_path_buf()
            .join("include")
            .to_str()
            .unwrap()
            .to_string(),
    );
    include_dirs.push(getenv("OUT_DIR"));

    let mut include_files: Vec<_> = include_files
        .iter()
        .map(|path| format!("{}{}", sdk_path, path))
        .collect();

    let mut config_headers: Vec<_> = CONFIG_HEADERS.iter().map(|s| s.to_string()).collect();
    include_files.append(&mut config_headers);

    let sdk_c_sources: Vec<_> = sdk_c_sources
        .iter()
        .filter_map(|path| {
            if cfg!(feature = "no_main") && path.ends_with("arch_main.c") {
                return None;
            }
            if cfg!(not(feature = "driver_spi")) && path.ends_with("spi_531.c") {
                return None;
            }
            if cfg!(not(feature = "driver_spi_flash")) && path.ends_with("spi_flash.c") {
                return None;
            }
            Some(format!("{}{}", sdk_path, path))
        })
        .collect();

    let sdk_asm_sources: Vec<_> = SDK_ASM_SOURCES
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
        sdk_c_sources,
        sdk_asm_sources,
        defines,
    )
}

fn generate_bindings(
    include_dirs: &Vec<String>,
    include_files: &Vec<String>,
    defines: &Vec<(String, Option<String>)>,
    rustify_enums: &Vec<&str>,
) {
    let sysroot = cc::Build::new()
        .get_compiler()
        .to_command()
        .arg("--print-sysroot")
        .output()
        .unwrap();
    let sysroot = PathBuf::from(std::str::from_utf8(&sysroot.stdout).unwrap().trim_end());
    let mut builder = bindgen::Builder::default()
        .header("bindings.h")
        .wrap_static_fns(true)
        .ctypes_prefix("cty")
        .use_core()
        .size_t_is_usize(true)
        .clang_arg("-D__SOFTFP__")
        .clang_arg(format!("--sysroot={}", sysroot.to_str().unwrap()))
        .clang_arg("-Wno-expansion-to-defined");

    for (key, value) in defines {
        if let Some(value) = value {
            builder = builder.clang_arg(format!("-D{}={}", key, value));
        } else {
            builder = builder.clang_arg(format!("-D{}", key));
        }
    }

    for inc_dir in include_dirs {
        builder = builder.clang_arg(format!("-I{}", translate_path(inc_dir)));
    }

    for inc_file in include_files {
        builder = builder.clang_args(vec!["-include", &translate_path(inc_file)]);
    }

    for re in rustify_enums {
        builder = builder.rustified_enum(re)
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(getenv("OUT_DIR"));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn compile_sdk(
    include_dirs: &[String],
    include_files: &[String],
    defines: &[(String, Option<String>)],
    sdk_c_sources: &[String],
    sdk_asm_sources: &[String],
) {
    // Assembly files, we don't want to use the -include flag here
    let asm_objs = cc::Build::new()
        .files(sdk_asm_sources)
        .flag("-specs=nano.specs")
        .flag("-specs=nosys.specs")
        .flag("-flto")
        .define("__DA14531__", None)
        .compile_intermediates();

    let mut cc_builder = cc::Build::new();

    let mut cc_builder = cc_builder
        .debug(true)
        .target("thumbv6m-none-eabi")
        .flag("-march=armv6-m")
        .flag("-Wno-expansion-to-defined")
        .flag("-Wno-unused-parameter")
        .flag("-fstack-usage")
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        .flag("-specs=nano.specs")
        .flag("-specs=nosys.specs")
        .opt_level_str("z")
        .flag("-flto")
        .archiver("arm-none-eabi-gcc-ar")
        .ranlib("arm-none-eabi-gcc-ranlib")
        .link_lib_modifier("+whole-archive");

    for inc_dir in include_dirs {
        cc_builder = cc_builder.include(translate_path(inc_dir));
    }

    for inc_file in include_files {
        cc_builder = cc_builder.flag(&format!("-include{}", translate_path(inc_file)));
    }

    for (key, value) in defines {
        cc_builder = cc_builder.define(key, value.as_ref().map(|v| v.as_str()));
    }

    for file in sdk_c_sources {
        cc_builder = cc_builder.file(translate_path(file));
    }

    for file in asm_objs {
        cc_builder.object(file);
    }

    // Bindgen creates a C-file for static fns
    {
        let mut path = env::temp_dir();
        path.extend(["bindgen", "extern.c"]);
        cc_builder.file(translate_path(path.to_str().unwrap()));
        cc_builder.include(translate_path(&getenv("CARGO_MANIFEST_DIR")));
    }

    cc_builder.compile("da14531_sdk");
}

// Panics in case something is wrong
fn check_env() -> Result<(), String> {
    let expected_target = "thumbv6m-none-eabi";
    let target = getenv("TARGET");
    if target != expected_target {
        return Err(format!(
            "Invalid target `{target}` for this crate. Only `{expected_target}` is valid"
        ));
    }

    if env::var("SDK_PATH").is_err() {
        return Err("SDK_PATH is missing. Must be set to DA145XX SDK path".into());
    }

    #[derive(PartialEq, Eq)]
    struct SdkVersion([u32; 4]);

    fn parse_version(path: &Path) -> Result<SdkVersion, String> {
        let buf = {
            let mut file = File::open(path).map_err(|e| e.to_string())?;
            let mut buf = String::new();
            file.read_to_string(&mut buf).map_err(|e| e.to_string())?;
            buf
        };
        let first_line = buf.lines().next().ok_or("version file empty")?;
        const PREFIX: &str = "#define SDK_VERSION \"v_";
        const SUFFIX: &str = "\"";
        if let Some(s) = first_line.strip_prefix(PREFIX) {
            if let Some(s) = s.strip_suffix(SUFFIX) {
                let mut parts = [0; 4];
                for (i, part) in s.split(".").enumerate() {
                    parts[i] = part.parse().map_err(|e: ParseIntError| e.to_string())?;
                }
                Ok(SdkVersion(parts))
            } else {
                Err(format!("could not strip suffix {SUFFIX}"))
            }
        } else {
            Err(format!("could not strip prefix {PREFIX}"))
        }
    }

    let path = PathBuf::from_iter([&getenv("SDK_PATH"), "sdk/platform/include/sdk_version.h"]);
    match parse_version(&path) {
        Ok(version) => {
            if version != SdkVersion([6, 0, 22, 1401]) {
                println!("cargo::warning=SDK has only been tested with 6.0.22.1401");
            }
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}

fn generate_linker_script(include_dirs: &[String], defines: &[(String, Option<String>)]) {
    let path = PathBuf::from_iter([
        &getenv("SDK_PATH"),
        "sdk",
        "common_project_files",
        "ldscripts",
        "ldscript_DA14531.lds.S",
    ]);
    let mut cc_builder = cc::Build::new();
    cc_builder
        .file(path.as_os_str())
        .flag("-Wno-expansion-to-defined")
        .flag("-Wno-unused-parameter")
        .flag("-specs=nano.specs")
        .flag("-specs=nosys.specs");

    for inc_dir in include_dirs {
        cc_builder.include(translate_path(inc_dir));
    }

    for (key, value) in defines {
        cc_builder.define(key, value.as_ref().map(|v| v.as_str()));
    }
    let content = cc_builder.expand();
    let out_path = PathBuf::from_iter([&getenv("OUT_DIR"), "ldscript_DA14531.lds"]);
    let mut fh = File::create(out_path).unwrap();
    fh.write_all(&content).unwrap();

    // Search path to ldscript_DA14531.lds
    // Picking the linker script is done by the consuming crate
    println!("cargo::rustc-link-search={}", getenv("OUT_DIR"));
    // Search path to da14531_symbols.lds, used by the linker script
    let path = PathBuf::from_iter([&getenv("SDK_PATH"), "sdk", "common_project_files", "misc"]);
    let path = path.to_str().unwrap();
    println!("cargo::rustc-link-search={path}");
}

fn main() {
    if let Err(e) = check_env() {
        println!("cargo::warning={}", e);
        return;
    }

    let (include_dirs, include_files, sdk_c_sources, sdk_asm_sources, defines) = setup_build();

    generate_user_config();
    generate_user_modules_config();
    generate_user_profiles_config();
    generate_da14531_config_basic();
    generate_da14531_config_advanced();

    generate_bindings(
        &include_dirs,
        &include_files,
        &defines,
        &vec![
            "hl_err",
            "process_event_response",
            "syscntl_dcdc_level_t",
            "APP_MSG",
        ],
    );

    compile_sdk(
        &include_dirs,
        &include_files,
        &defines,
        &sdk_c_sources,
        &sdk_asm_sources,
    );

    // Also link with vendor provided binary blobs :'(
    println!("cargo::rustc-link-lib=static:+whole-archive,+verbatim=da14531.a");
    // Search path to binary lib above
    let path = PathBuf::from_iter([
        &getenv("SDK_PATH"),
        "sdk",
        "platform",
        "system_library",
        "output",
        "IAR",
    ]);
    println!("cargo::rustc-link-search=native={}", path.to_str().unwrap());

    generate_linker_script(&include_dirs, &defines);

    println!("cargo:rerun-if-changed=bindings.h");
    println!("cargo:rerun-if-changed=build.rs");

    println!(
        "cargo:rerun-if-changed={}",
        &translate_path("include/da14531_config_basic.h.in")
    );
    println!(
        "cargo:rerun-if-changed={}",
        &translate_path("include/da14531_config_advanced.h.in")
    );
    println!(
        "cargo:rerun-if-changed={}",
        &translate_path("include/user_callback_config.h.in")
    );
    println!(
        "cargo:rerun-if-changed={}",
        &translate_path("include/user_custs_config.h.in")
    );
    println!(
        "cargo:rerun-if-changed={}",
        &translate_path("include/user_periph_setup.h.in")
    );
}

fn translate_path(path: &str) -> String {
    if cfg!(windows) {
        path.replace("/", "\\")
    } else {
        path.to_string()
    }
}
