type PaPwrLevel = crate::bindings::rf_tx_pwr_lvl_t;

pub fn rf_pa_pwr_set(level: PaPwrLevel) {
    unsafe {
        crate::bindings::rf_pa_pwr_set(level);
    }
}
