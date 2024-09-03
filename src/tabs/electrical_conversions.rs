use slint::ComponentHandle;

pub fn init_electrical_conversions() {
    unimplemented!();
}

pub fn on_dbv_input() {   
    unimplemented!();
    // value= value
    // z= impedance
    // val_results = []
    // val_results.append(f"Z={z}, {value} dBV is :")
    // val_results.append(f"{value + 120} dBµV")
    // val_results.append(f"{10 ** ((value)/20)} V")
    // val_results.append(f"{10 ** ((value/20))*1e3} mV")
    // val_results.append(f"{10 ** ((value/20))*1e6} µV")
    // val_results.append(f"{20*math.log10(10**((value)/20))-10*math.log10(z)} dBW")
    // val_results.append(f"{20*math.log10(10**(value/20))-10*math.log10(z)+30} dBmW")
    // val_results.append(f"{20*math.log10(10**(value/20))-10*math.log10(z)+60} dBµW")
    // val_results.append(f"{10**(value/10) **2 / z} W")
    // val_results.append(f"{10**(value/10) **2 / z * 1e3} mW")
    // val_results.append(f"{10**(value/10) **2 / z * 1e6} µW")
    // val_results.append(f"{value - 20*math.log10(z)}dBA")
    // val_results.append(f"{value +120 - 20*math.log10(z)}dBµA")
    // val_results.append(f"{10**((value - 20*math.log10(z))/20)}A")
    // val_results.append(f"{10**((value + 60 - 20*math.log10(z))/20)}mA")
    // val_results.append(f"{10**((value + 120 - 20*math.log10(z))/20)}µA")
}

pub fn on_dba_input() {   
    unimplemented!();
}

pub fn on_dbw_input() {   
    unimplemented!(); 
}

pub fn on_dbuv_input() {
    unimplemented!();
    // value = value
    // z = impedance
    // val_results = []
    // val_results.append(f"Z={z}, {value} dBµV is :")
    // val_results.append(f"{value - 120} dBV")
    // val_results.append(f"{10 ** ((value-120)/20)} V")
    // val_results.append(f"{10 ** ((value/20))*1e-3} mV")
    // val_results.append(f"{10 ** ((value/20))} µV")
    // val_results.append(f"{20*math.log10(10**((value-120)/20))-10*math.log10(z)} dBW")
    // val_results.append(f"{20*math.log10(10**(value/20)*1e-3)-10*math.log10(z*1e3)} dBmW")
    // val_results.append(f"{20*math.log10(10**(value/20))-10*math.log10(z*1e6)} dBµW")
    // val_results.append(f"{(10 ** ((value-120)/20))**2 / z} W")
    // val_results.append(f"{(10 ** ((value-120)/20))**2 / z * 1e3} mW")
    // val_results.append(f"{(10 ** ((value-120)/20))**2 / z * 1e6} µW")
    // val_results.append(f"{107-120 - 20*math.log10(z)}dBA")
    // val_results.append(f"{value - 20*math.log10(z)}dBµA")
    // val_results.append(f"{10**((107-120 - 20*math.log10(z))/20)}A")
    // val_results.append(f"{10**((107 - 60 - 20*math.log10(z))/20)}mA")
    // val_results.append(f"{10**((107 - 0 - 20*math.log10(z))/20)}µA")
}

pub fn on_dbua_input() {
    unimplemented!();
}

pub fn on_dbmw_input() {
    unimplemented!();
}

pub fn on_v_input() {
    unimplemented!();
    // value = value
    // z = impedance
    // val_results = []
    // val_results.append(f"Z={z}, {value} V is :")
    // val_results.append(f"{20*log(value)} dBV")
    // val_results.append(f"{20*log(value/1e-6)} dBµV")
    // val_results.append(f"{value * 1e3} mV")
    // val_results.append(f"{value * 1e6} µV")
    // val_results.append(f"{20*math.log10(value)-10*math.log10(z)} dBW")
    // val_results.append(f"{20*math.log10(value) + 30 - 10*math.log10(z)} dBmW")
    // val_results.append(f"{20*math.log10(value) + 60 -10*math.log10(z)} dBµW")
    // val_results.append(f"{(10 ** ((value-120)/20))**2 / z} W")
    // val_results.append(f"{(10 ** ((value-120)/20))**2 / z * 1e3} mW")
    // val_results.append(f"{(10 ** ((value-120)/20))**2 / z * 1e6} µW")
    // val_results.append(f"{20*log(value) - 20*log(z)}dBA")
    // val_results.append(f"{20*log(value)+120 - 20*log(z)}dBµA")
    // val_results.append(f"{value / z}A")
    // val_results.append(f"{value / z * 1e3}mA")
    // val_results.append(f"{value / z * 1e6}µA")
}

pub fn on_a_input() {
    unimplemented!();
}

pub fn on_w_input() {
    unimplemented!();
}

pub fn on_mv_input() {
    unimplemented!();
}

pub fn on_ma_input() {
    unimplemented!();
}

pub fn on_mw_input() {
    unimplemented!();
}

pub fn on_uv_input() {
    unimplemented!();
}

pub fn on_ua_input() {
    unimplemented!();
}

pub fn on_uw_input() {
    unimplemented!();
} 

pub fn change_waveform_icon() {
    unimplemented!();
}

