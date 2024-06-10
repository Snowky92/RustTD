use bevy::prelude::*;

use super::Money;

#[derive(Component)]
pub struct MoneyText;

pub fn ui_setup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("OR : ", TextStyle {
                font_size: 60.0,
                ..default()
            }),
            TextSection::from_style(
                TextStyle {
                    font_size: 60.0,
                    color: Color::GOLD,
                    ..default()
                }
            )
        ]).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            right: Val::Px(20.0),
            ..default()
        }),
        MoneyText));
}

pub fn update_money_text(
    mut query: Query<&mut Text, With<MoneyText>>,
    money: Res<Money>
) {

    for mut text in &mut query {
        text.sections[1].value = format!("{:?}", money.amount);
    }
} 