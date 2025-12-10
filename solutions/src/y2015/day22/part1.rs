/// ### Solution for Part 1
/// This is a continuation of the RPG game from 21.
/// However it is more complicated. In this we have 
/// effects and spells of various kinds, costing mana.
/// This problem asks us to find the lowest mana required 
/// to defeat the boss.
/// 
/// #### Rust Implementation Details
/// We model the problem with spells, user and boss. 
/// User has its HP, mana, current armor, and active
/// effects.
/// Boss has HP and damage.
/// 
/// We start with the user's turn and create a simulation
/// tree using all possible spells.
/// Each node of the tree then has active effects applied 
/// before the boss takes its turn to deal
/// damage depending on the armor.
/// Then the effects are applied again and the player takes 
/// its turn. 
/// The minimum mana spent is intially 0, and increases on 
/// each use, being stored with the nodes.
/// Each node generated gets stored into a state vector, which
/// then pops the next node to continue the branch evaluation.
/// Once we find a minimum mana when boss is defeated we use 
/// it to prune further branches when the mana cost exceeds 
/// this value. 
/// Finally, we find the min mana after all the states have 
/// been exhausted.

#[derive(Clone, Debug)]
struct Spell<'a> {
    name: &'a str,
    turns_left: u8,
    armor: u32,
    damage: i32,
    mana: u32,
    heals: u32,
    cost: u32
}

#[derive(Clone, Debug)]
struct State<'a> {
    mana_spent: u32,
    active_effects: Vec<Spell<'a>>,
    player_hp: i32,
    player_mana: u32,
    player_armor: u32,
    boss_hp: i32
}

const SPELL_NAMES: [&str; 5] = ["Magic Missile", "Drain", "Shield", "Poison", "Recharge"];

fn get_spells() -> Vec<Spell<'static>> {
    return vec![
        Spell {
            name: SPELL_NAMES[0],
            turns_left: 0,
            armor: 0,
            damage: 4,
            mana: 0,
            heals: 0,
            cost: 53
        },
        Spell {
            name: SPELL_NAMES[1],
            turns_left: 0,
            armor: 0,
            damage: 2,
            mana: 0,
            heals: 2,
            cost: 73
        },
        Spell {
            name: SPELL_NAMES[2],
            turns_left: 6,
            armor: 7,
            damage: 0,
            mana: 0,
            heals: 0,
            cost: 113
        },
        Spell {
            name: SPELL_NAMES[3],
            turns_left: 6,
            armor: 0,
            damage: 3,
            mana: 0,
            heals: 0,
            cost: 173,
        },
        Spell {
            name: SPELL_NAMES[4],
            turns_left: 5,
            armor: 0,
            damage: 0,
            mana: 101,
            heals: 0,
            cost: 229,
        }
    ];
}

pub fn solve(hp: i32, dmg: i32) {
    let spells = get_spells();
    let boss_damage = dmg;
    let mut min_heap = vec![];
    let mut state_count = 0;
    min_heap.push(State {
        mana_spent: 0,
        active_effects: vec![],
        player_hp: 50,
        player_armor: 0,
        player_mana: 500,
        boss_hp: hp
    });

    let mut min_mana = u32::MAX;

    while let Some(mut state) = min_heap.pop() {
        state_count += 1;
        let boss_dead = apply_effects(&mut state);

        if boss_dead && state.mana_spent < min_mana {
            min_mana = state.mana_spent;
            continue;
        } 

        if state_count > 1 {
            state.player_hp -= calculate_damage(boss_damage, state.player_armor as i32);
            if state.player_hp <= 0 {
                continue;
            }
        }

        let boss_dead = apply_effects(&mut state);

        if boss_dead && state.mana_spent < min_mana {
            min_mana = state.mana_spent;
            continue;
        } 

        
        for spell in &spells {
            if can_cast(spell, &state, &min_mana) {
                match spell.name {
                    "Magic Missile" | "Drain" => {                                
                        if state.boss_hp - spell.damage as i32 <= 0 {
                            min_mana = state.mana_spent + spell.cost;
                            continue;
                        }

                        let mut new_state = state.clone();
                        new_state.boss_hp -= spell.damage as i32;
                        new_state.mana_spent += spell.cost;
                        new_state.player_mana -= spell.cost;
                        new_state.player_hp += spell.heals as i32;
                        min_heap.push(new_state);
                    },
                    "Shield" | "Poison" | "Recharge" => {
                        let mut new_state = state.clone();
                        new_state.active_effects.push(spell.clone());
                        new_state.mana_spent += spell.cost;
                        new_state.player_mana -= spell.cost;
                        min_heap.push(new_state);
                    },
                    _ => {},
                }
            }
        }
    }
    println!("States visited: {state_count}");
    println!("Minimum mana used: {min_mana}");
}

fn can_cast(spell: &Spell, state: &State, min_mana: &u32) -> bool {
    let spell_in_state = state.active_effects.iter().find(|effect| effect.name == spell.name);

    if let Some(_) = spell_in_state {
        return false;
    } else {
        if state.player_mana < spell.cost {
            return false;
        } else if state.mana_spent + spell.cost >= *min_mana {
            return false;
        } else {
            return true;
        }
    }
}

fn calculate_damage(boss_damage: i32, player_armor: i32) -> i32 {
    boss_damage - player_armor
}

fn apply_effects(state: &mut State) -> bool {
    for effect in &mut state.active_effects {
        match effect.name {
            "Shield" => {
                state.player_armor = effect.armor;
                if effect.turns_left == 1 {
                    state.player_armor = 0;
                }
            },
            "Poison" => {
                state.boss_hp -= effect.damage;

            },
            "Recharge" => {
                state.player_mana += effect.mana;
            },
            _ => {}
        }
        effect.turns_left -= 1;
    }

    if state.boss_hp <= 0 {
        return true;
    } else {
        state.active_effects.retain(|effect| effect.turns_left > 0);
        return false;
    }
}