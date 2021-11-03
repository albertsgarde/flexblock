use legion::{Entity, Resources, Schedule, World, query, system, IntoQuery};

#[system(for_each)]
fn increment(value: &mut u32, #[resource] increment: &u32) {
    *value += increment;
}

fn main() {
    

    let mut world = World::default();
    let mut resources = Resources::default();
    resources.insert(2u32);

    let mut query = <&u32>::query();

    println!("First.");
    query.for_each(&world, |value| println!("{:?}", value) );

    let mut schedule = Schedule::builder().add_system(increment_system()).build();

    schedule.execute(&mut world, &mut resources);
    
    println!("Second.");
    query.for_each(&world, |value| println!("{:?}", value) );

    let entity = world.push((25u32,));
    let entry = world.entry(entity).unwrap();
    println!("{:?} has {:?}", entity, entry.archetype().layout().component_types());
    
    println!("Third.");
    query.for_each(&world, |value| println!("{:?}", value) );


    schedule.execute(&mut world, &mut resources);
    
    println!("Fourth.");
    query.for_each(&world, |value| println!("{:?}", value) );
    
    resources.insert(5u32);


    schedule.execute(&mut world, &mut resources);
    
    println!("Fifth.");
    query.for_each(&world, |value| println!("{:?}", value) );

    
}
