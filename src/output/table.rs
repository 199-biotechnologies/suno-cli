use comfy_table::{ContentArrangement, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

use crate::api::types::{BillingInfo, Clip, LyricsResult, Model};

pub fn clips(clips: &[Clip]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["ID", "Title", "Status", "Model", "Duration", "Tags"]);

    for c in clips {
        let duration = c
            .metadata
            .duration
            .map(|d| format!("{:.0}s", d))
            .unwrap_or_default();
        let tags = c.metadata.tags.as_deref().unwrap_or("-");
        let short_id = if c.id.len() > 8 { &c.id[..8] } else { &c.id };
        table.add_row(vec![
            short_id,
            &c.title,
            &c.status,
            &c.model_name,
            &duration,
            tags,
        ]);
    }
    println!("{table}");
}

pub fn billing(info: &BillingInfo) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Field", "Value"]);

    table.add_row(vec!["Plan", &info.plan.name]);
    table.add_row(vec!["Credits Left", &info.total_credits_left.to_string()]);
    table.add_row(vec![
        "Monthly Usage",
        &format!("{} / {}", info.monthly_usage, info.monthly_limit),
    ]);
    table.add_row(vec!["Active", &info.is_active.to_string()]);
    table.add_row(vec!["Period", &info.period]);
    if let Some(ref renew) = info.renews_on {
        table.add_row(vec!["Renews On", renew]);
    }
    println!("{table}");
}

pub fn models(models: &[Model]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            "Name",
            "Key",
            "Default",
            "Max Prompt",
            "Max Tags",
            "Description",
        ]);

    for m in models {
        if !m.can_use {
            continue;
        }
        table.add_row(vec![
            &m.name,
            &m.external_key,
            &if m.is_default_model {
                "yes".into()
            } else {
                String::new()
            },
            &m.max_lengths.prompt.to_string(),
            &m.max_lengths.tags.to_string(),
            &m.description,
        ]);
    }
    println!("{table}");
}

pub fn lyrics(result: &LyricsResult) {
    println!("Title: {}\n", result.title);
    println!("{}", result.text);
    if !result.tags.is_empty() {
        println!("\nSuggested style: {}", result.tags.join(", "));
    }
}
