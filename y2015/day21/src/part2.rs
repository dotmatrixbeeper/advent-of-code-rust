/// ### Solution for Part 2
/// The second part of the problem asks us to find the 
/// max gold instead.
/// 
/// #### Rust Implementation Details
/// The implementation remains the same except we change 
/// the min finder to max finder.

use crate::Item;

pub fn solve(weapons: &Vec<Item>, armors: &Vec<Item>, rings: &Vec<Item>) {

    let enemy_damage = 8;
    let enemy_armor = 2;
    let mut max_cost = 0;

    let zero_cost_armor = Item {
                            cost: 0,
                            damage: 0,
                            armor: 0
                        };
    let zero_cost_ring = Item {
                            cost: 0,
                            damage: 0,
                            armor: 0,
                        };

    for weapon in weapons {
        for armor_index in 0..=armors.len() {
            for ring_index_1 in 0..=rings.len() {
                for ring_index_2 in 0..=rings.len() {

                    if ring_index_1 > 0 && ring_index_2 > 0 && ring_index_1 == ring_index_2 {
                        continue;
                    }

                    let armor;
                    if armor_index == 0 {
                        armor = &zero_cost_armor;
                    } else {
                        armor = &armors[armor_index - 1];
                    }
                    let  ring_1;
                    if ring_index_1 == 0 {
                        ring_1 = &zero_cost_ring;
                    } else {
                        ring_1 = &rings[ring_index_1 - 1];
                    }

                    let  ring_2;
                    if ring_index_2 == 0 {
                        ring_2 = &zero_cost_ring;
                    } else {
                        ring_2 = &rings[ring_index_2 - 1];
                    }

                    let my_damage = weapon.damage + ring_1.damage + ring_2.damage;
                    let my_armor = armor.armor + ring_1.armor + ring_2.armor;
                    let cost = weapon.cost + armor.cost + ring_1.cost + ring_2.cost;
                    let mut my_hit_points = 100;
                    let mut enemy_hit_points = 109;
                    let mut my_turn = true;
                    while my_hit_points > 0 && enemy_hit_points > 0 {
                        if my_turn {
                            if my_damage > enemy_armor {
                                enemy_hit_points -= my_damage - enemy_armor;
                            } else {
                                enemy_hit_points -= 1;
                            }
                        } else {
                            if enemy_damage > my_armor {
                                my_hit_points -= enemy_damage - my_armor;
                            } else {
                                my_hit_points -= 1;
                            }
                        }
                        my_turn = !my_turn;
                    }

                    if my_hit_points > enemy_hit_points {
                        continue;
                    }

                    if cost > max_cost {
                        max_cost = cost;
                    }
                }
            }
        }
    }

    println!("Max cost to still lose: {max_cost}");
}