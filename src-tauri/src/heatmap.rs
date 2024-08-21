use serde_json::Value;

pub fn generate_prayer_heatmap_svg(prayer_data: &Value, description: String) -> String {
    let days = prayer_data["data"].as_array().unwrap();
    let prayers = ["Fajr", "Dhuhr", "Asr", "Maghrib", "Isha"];
    let cell_size = 30;
    let padding = 20;
    let label_padding_x = 100;
    let label_padding_y = 100;
    let width = prayers.len() as i32 * (cell_size + padding) + label_padding_x + 300;
    let height = days.len() as i32 * (cell_size + padding) + label_padding_y + 10;

    let mut svg_content = String::new();

    // SVG opening tag and style
    svg_content.push_str(&format!(
        "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">",
        width, height
    ));
    svg_content.push_str(
        "<style>
            text { font-family: Arial, sans-serif; fill: white; }
            .title { font-size: 20px; font-weight: bold; }
            .axis-label { font-size: 14px; font-weight: bold; }
            .legend-text { font-size: 12px; }
            rect { stroke: #444; stroke-width: 1; }
        </style>"
    );

    // Background rectangle
    svg_content.push_str(
        "<rect width=\"100%\" height=\"100%\" fill=\"#000000\" rx=\"15\" ry=\"15\"/>"
    );

    // Add title
    svg_content.push_str(&format!(
        "<text x=\"{}\" y=\"40\" class=\"title\" text-anchor=\"middle\">{}</text>",
        width / 2, description
    ));

    // Add x-axis labels (prayer names)
    for (i, &prayer) in prayers.iter().enumerate() {
        let x = i as i32 * (cell_size + padding) + label_padding_x;
        let y = label_padding_y - 20;
        svg_content.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" class=\"axis-label\" text-anchor=\"middle\">{}</text>",
            x + cell_size / 2, y, prayer
        ));
    }

    // Add y-axis labels (dates in DD-MM-YYYY format)
    for (i, day) in days.iter().enumerate() {
        let date = day["date"].as_str().unwrap();
        let formatted_date = format!("{}-{}-{}", &date[8..10], &date[5..7], &date[0..4]);
        let x = label_padding_x - 10;
        let y = i as i32 * (cell_size + padding) + label_padding_y;
        svg_content.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" class=\"axis-label\" text-anchor=\"end\" alignment-baseline=\"middle\">{}</text>",
            x, y + cell_size / 2, formatted_date
        ));
    }

    // Draw the heatmap cells with rounded borders
    for (i, day) in days.iter().enumerate() {
        let row = i as i32;
        let prayers_done = [
            day["fajr"].as_bool().unwrap_or(false),
            day["dhuhr"].as_bool().unwrap_or(false),
            day["asr"].as_bool().unwrap_or(false),
            day["maghrib"].as_bool().unwrap_or(false),
            day["isha"].as_bool().unwrap_or(false),
        ];
        for (j, &done) in prayers_done.iter().enumerate() {
            let col = j as i32;
            let color = if done { "#21c35d" } else { "#da204c" };
            let x = col * (cell_size + padding) + label_padding_x;
            let y = row * (cell_size + padding) + label_padding_y;
            svg_content.push_str(&format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" rx=\"5\" ry=\"5\"/>",
                x, y, cell_size, cell_size, color
            ));
        }
    }

    // Draw the legend
    let legend_x = width - 170;
    let legend_y = height - 100;
    let legend_items = vec![
        ("#21c35d", "Prayer Done"),
        ("#da204c", "Prayer Not Done"),
    ];
    svg_content.push_str(&format!(
        "<rect x=\"{}\" y=\"{}\" width=\"140\" height=\"75\" fill=\"#222\" stroke=\"#444\" stroke-width=\"1\" rx=\"10\" ry=\"10\"/>",
        legend_x - 5, legend_y - 5
    ));
    for (i, &(color, label)) in legend_items.iter().enumerate() {
        let y = legend_y + i as i32 * (cell_size + 5);
        svg_content.push_str(&format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" rx=\"5\" ry=\"5\"/>",
            legend_x, y, cell_size, cell_size, color
        ));
        svg_content.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" class=\"legend-text\" text-anchor=\"start\" alignment-baseline=\"middle\">{}</text>",
            legend_x + cell_size + 10, y + cell_size / 2, label
        ));
    }

    svg_content.push_str("</svg>");
    svg_content
}
