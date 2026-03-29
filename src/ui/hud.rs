use bevy::prelude::*;

#[derive(Component)] pub struct StatusText;
#[derive(Component)] pub struct StatsText;
#[derive(Component)] pub struct HelpText;

pub fn spawn_hud(commands: &mut Commands) {
    // Root node: full screen, column layout
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(Val::Px(16.0)),
            ..default()
        })
        .with_children(|parent| {
            // ── Top bar ───────────────────────────────────────────────────────
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                })
                .with_children(|row| {
                    // Status indicator
                    row.spawn((
                        Text::new("● UNSOLVED"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.3, 0.3)),
                        StatusText,
                    ));

                    // Stats panel
                    row.spawn((
                        Text::new(""),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        StatsText,
                    ));
                });

            // ── Bottom help bar ───────────────────────────────────────────────
            parent.spawn((
                Text::new(
                    "[S] Scramble   [Space] Solve   [R/L/U/D/F/B] Manual move   [Shift] Inverse   [Mouse] Rotate view"
                ),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                HelpText,
            ));
        });
}
