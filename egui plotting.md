## Sine wave plot

```
let sin: PlotPoints = (0..1000)
        .map(|i| {
            let x = i as f64 * 0.01;
            [x, x.sin()]
        })
        .collect();
    let line = Line::new(sin);
    Plot::new("my_plot")
        .view_aspect(2.0)
        .height(300.0)
        .show(ui, |plot_ui| plot_ui.line(line));
```

## BarChart (histogram)

```
let histogram_data = vec![
        (0.5, 50.0, 0.2, Color32::BROWN.gamma_multiply(0.5)),
        (1.5, 10.0, 0.4, Color32::PURPLE.gamma_multiply(0.5)),
        (2.5, 7.0, 0.6, Color32::ORANGE.gamma_multiply(0.5)),
        (3.5, 12.0, 0.3, Color32::BLUE.gamma_multiply(0.5)),
        (4.5, 50.0, 0.8, Color32::RED.gamma_multiply(0.5)),
    ];

    let bars: Vec<Bar> = histogram_data
        .iter()
        .map(|(x, y, w, c)| {
            Bar::new(*x, *y)
                .width(*w)
                .stroke(Stroke::new(1.0, Color32::DARK_GRAY))
                .fill(*c)
        })
        .collect();

    let chart = BarChart::new("Plot name", bars);

    Plot::new("Histogram")
        .allow_scroll(false)
        .width(800.0)
        .height(400.0)
        .legend(Legend::default())
        .set_margin_fraction(vec2(0.1, 0.1))
        .show(ui, |plot_ui| {
            plot_ui.bar_chart(chart);
        });
```
