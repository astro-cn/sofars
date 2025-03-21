use sofars::eph::*;

#[test]
fn test_epv00() {
    match epv00(2400000.5, 53411.52501161) {
        Some((pvh, pvb)) => {
            assert!((pvh[0][0] - -0.7757238809297706813).abs() < 1e-14, "ph(x)");
            assert!((pvh[0][1] - 0.5598052241363340596).abs() < 1e-14, "ph(y)");
            assert!((pvh[0][2] - 0.2426998466481686993).abs() < 1e-14, "ph(z)");
        
            assert!((pvh[1][0] - -0.01091891824147313846).abs() < 1e-15, "vh(x)");
            assert!((pvh[1][1] - -0.01247187268440845008).abs() < 1e-15, "vh(y)");
            assert!((pvh[1][2] - -0.005407569418065039061).abs() < 1e-15, "vh(z)");
        
            assert!((pvb[0][0] - -0.7714104440491111971).abs() < 1e-14, "pb(x)");
            assert!((pvb[0][1] - 0.5598412061824171323).abs() < 1e-14, "pb(y)");
            assert!((pvb[0][2] - 0.2425996277722452400).abs() < 1e-14, "pb(z)");
        
            assert!((pvb[1][0] - -0.01091874268116823295).abs() < 1e-15, "vb(x)");
            assert!((pvb[1][1] - -0.01246525461732861538).abs() < 1e-15, "vb(y)");
            assert!((pvb[1][2] - -0.005404773180966231279).abs() < 1e-15, "vb(z)");
        },
        None => {
            panic!("Error: {:?}", "epv00() returned None");
        }
    }
}