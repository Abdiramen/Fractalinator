pub fn interpolate_rgb(colors: &[(u8, u8, u8)], positions: Vec<f64>) -> Vec<(u8, u8, u8)> {
    assert_eq!(colors.len(), positions.len());
    let mut reds = vec![0.0; positions.len()];
    let mut greens = vec![0.0; positions.len()];
    let mut blues = vec![0.0; positions.len()];

    for point in 0..positions.len() {
        reds[point] = colors[point].0 as f64;
        greens[point] = colors[point].1 as f64;
        blues[point] = colors[point].2 as f64;
    }
    let red_fn = monotone_cubic_interpolation(positions.clone(), reds);
    let green_fn = monotone_cubic_interpolation(positions.clone(), greens);
    let blue_fn = monotone_cubic_interpolation(positions.clone(), blues);

    let mut rgb_vec: Vec<(u8, u8, u8)> = Vec::new();
    for i in 0..2048 {
        let v = i as f64 * (positions[positions.len() - 1] / 2048 as f64);
        let r = red_fn(v) as u8;
        let g = green_fn(v) as u8;
        let b = blue_fn(v) as u8;

        rgb_vec.push((r, g, b));
    }

    return rgb_vec;
}

pub fn monotone_cubic_interpolation(xs: Vec<f64>, ys: Vec<f64>) -> impl Fn(f64) -> f64 {
    assert_eq!(xs.len(), ys.len());

    // Get consecutive differences and slopes
    let mut dxs: Vec<f64> = Vec::new();
    let mut dys: Vec<f64> = Vec::new();
    let mut ms: Vec<f64> = Vec::new();
    for i in 0..xs.len() - 1 {
        let dx = xs[i + 1] - xs[i];
        let dy = ys[i + 1] - ys[i];
        dxs.push(dx);
        dys.push(dy);
        ms.push(dy / dx);
    }

    // Get degree 1 coefficients
    let mut c1s: Vec<f64> = vec![ms[0]];
    for i in 0..dxs.len() - 1 {
        let m = ms[i];
        let m_next = ms[i + 1];
        if m * m_next <= 0.0 {
            c1s.push(0.0);
        } else {
            let dx = dxs[i];
            let dx_next = dxs[i + 1];
            let common = dx + dx_next;
            c1s.push(3.0 * common / ((common + dx_next) / m + (common + dx) / m_next));
        }
    }
    c1s.push(ms[ms.len() - 1]);

    // Get degree-2 and degree-3 coefficients
    let mut c2s: Vec<f64> = Vec::new();
    let mut c3s: Vec<f64> = Vec::new();
    for i in 0..c1s.len() - 1 {
        let c1 = c1s[i];
        let m = ms[i];
        let inv_dx = 1.0 / dxs[i];
        let common = c1 + c1s[i + 1] - m - m;
        c2s.push((m - c1 - common) * inv_dx);
        c3s.push(common * inv_dx * inv_dx);
    }

    // create 1024 values
    move |x: f64| {
        let mut i = xs.len() - 1;
        let mut p_val;
        if x == xs[i] {
            return ys[i];
        } else {
            // Search for the interval x is in, returning the corresponding y if x is one of the
            // original xs
            let mut low: usize = 0;
            let mut mid: usize;
            let mut high: usize = c3s.len() - 1;

            while low <= high {
                mid = ((high + low) as f64 * 0.5).floor() as usize;
                let x_here = xs[mid];
                if x_here < x {
                    low = mid + 1;
                } else if x_here > x {
                    high = mid - 1;
                } else {
                    return ys[mid];
                }
            }
            if high > 0 {
                i = high;
            } else {
                i = 0;
            }
            let diff = x - xs[i];
            let diff_sq = diff * diff;
            p_val = ys[i] + c1s[i] * diff + c2s[i] * diff_sq + c3s[i] * diff * diff_sq;
            if p_val < 0.0 {
                p_val = 0.0;
            }
        }
        return p_val;
    }
}
