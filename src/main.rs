use im_disperser::DisperserPlugin;
use nih_plug::prelude::*;

fn main() {
    nih_export_standalone::<DisperserPlugin>();
}
