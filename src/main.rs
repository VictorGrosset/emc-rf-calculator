mod tabs;

slint::include_modules!();

// TODO : Define MainWindow constraints, icon, name, etc.
// TODO : Enhance labeledTextInput Widget
// TODO : Create correct Waveform selector Widget with Horizontal box to have the value associated with the unit for conversion 
// TODO : Create correct Impedance selector Widget with Horizontal box to have the value associated with the unit for conversion 
// TODO : Create tabbed content application accross multiple .slint files
// TODO : Finish the layout
// TODO : Move all the logic to .rs files (No logic in .slint file except logic declaration)
// TODO : Implement update_from_XX every unit for conversion
// TODO : Add Magnetic field unit to conversion
// TODO : Implement impedance selector
// TODO : Implement Waveform selector
// TODO : Add reference to calculate difference in dB between two values

fn main() -> Result<(), slint::PlatformError> {
    let ui: AppWindow = AppWindow::new()?;

    ui.run()
}