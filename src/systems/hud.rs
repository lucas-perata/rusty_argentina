use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
#[read_component(Points)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query.iter(ecs).next().expect("Error HEALTH");

    let mut score_query = <&Points>::query().filter(component::<Player>());
    let player_score = score_query.iter(ecs).nth(0).unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(3);
    let (player, map_level) = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, player)| Some((*entity, player.map_level)))
        .unwrap();

    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, 1),
        format!("Nivel: {}", map_level + 1),
        ColorPair::new(YELLOW, BLACK),
    );

    let mut item_query = <(&Item, &Name, &Carried)>::query();
    let mut y = 3;

    item_query
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player)
        .for_each(|(_, name, _)| {
            draw_batch.print(Point::new(3, y), format!("{}: {}", y - 2, &name.0));
            y += 1;
        });
    if y > 3 {
        draw_batch.print_color(
            Point::new(3, 2),
            "Inventario",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    draw_batch.print_centered(1, "Explorá el calabozo con las flechitas.");
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!("Vida: {} / {}", player_health.current, player_health.max),
        ColorPair::new(WHITE, RED),
    );
    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, 3),
        format!("Puntaje: {}", player_score.current),
        ColorPair::new(WHITE, RED),
    );
    draw_batch.submit(10000).expect("Batch error");
}
