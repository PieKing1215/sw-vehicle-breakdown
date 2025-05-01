use std::{cmp::Ordering, collections::HashMap};

use chrono::{DateTime, Utc};
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    wasm_bindgen::JsCast,
    DragEvent,
};

use crate::data::{Definition, Vehicle};

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "IndexPageStateRx")]
struct IndexPageState {
    rom_date: DateTime<Utc>,
    status: String,
    definitions: HashMap<String, Definition>,
    vehicle: Option<Vehicle>,
}

fn index_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a IndexPageStateRx) -> View<G> {
    // let definitions = state.definitions.get();
    // let map: View<G> = View::new_fragment(
    //     definitions
    //         .iter()
    //         .map(|(k, v)| (k.clone(), v.clone()))
    //         .map(|(k, v)| {
    //             view! { cx,
    //                 p { (format!("{k}: {v:?}")) }
    //             }
    //         })
    //         .collect(),
    // );

    let cx_drop = cx.clone();
    let handle_drop = move |eve: web_sys::Event| {
        eve.prevent_default();
        let drag = eve.dyn_ref::<DragEvent>().unwrap();
        if let Some(data_transfer) = drag.data_transfer() {
            let items = data_transfer.items();
            for i in 0..items.length() {
                let item = items.get(i).unwrap();
                if item.kind() == "file" && item.type_() == "text/xml" {
                    let file = item.get_as_file().unwrap().unwrap();
                    state.status.set(format!("Loading {}", file.name()));

                    let status = state.status.clone();
                    let vehicle_rc = state.vehicle.clone();

                    spawn_local_scoped(cx_drop, async move {
                        let res = JsFuture::from(file.text()).await;

                        match res {
                            Err(e) => {
                                status.set(format!("Error loading file: {e:?}"));
                                vehicle_rc.set(None);
                            },
                            Ok(text) => {
                                let text = text.as_string().unwrap();

                                let vehicle_de = quick_xml::de::from_str(&text);

                                match vehicle_de {
                                    Err(e) => {
                                        status.set(format!("Error parsing file: {e:?}"));
                                        vehicle_rc.set(None);
                                    },
                                    Ok(vehicle) => {
                                        status.set(format!("Finished {}", file.name()));
                                        vehicle_rc.set(Some(vehicle));
                                    },
                                }
                            },
                        }
                    });
                    break;
                }
            }
        }
    };

    view! { cx,
        div (
            id = "main",
            on:drop = handle_drop,
            on:dragover = |eve: web_sys::Event| {
                eve.prevent_default();
            },
        ) {
            div (id = "content") {
                p { (state.status.get()) }
                ({
                    let vehicle = state.vehicle.get();
                    let vehicle_components = vehicle.as_ref().as_ref().map(|vehicle| {
                        let all_components = vehicle.bodies.bodies.iter().flat_map(|body| body.components.components.iter());
                        let map = all_components.fold(HashMap::new(), |mut map, cmp| {
                            let counts: &mut ComponentTotal = map.entry(cmp.definition.clone()).or_default();

                            counts.count += 1;

                            if let Some(mc) = &cmp.origin.microprocessor_definition {
                                counts.mass_mult += mc.width as u32 * mc.length as u32;
                            } else {
                                counts.mass_mult += 1;
                            }
                            
                            map
                        });

                        let defs = state.definitions.get();
                        let mut sorted = map.into_iter().map(|(component, counts)| {
                            let def = defs.get(&component);

                            (
                                def.map_or(component, |def| def.name.clone()),
                                counts.count,
                                def.map(|def| def.value),
                                def.map(|def| counts.count * def.value),
                                def.map(|def| def.mass),
                                def.map(|def| counts.mass_mult as f32 * def.mass),
                            )
                        }).collect::<Vec<_>>();
                        sorted.sort_by(|a, b| {
                            let (a_component, a_count, a_cost_per, a_total_cost, a_mass_per, a_total_mass) = a;
                            let (b_component, b_count, b_cost_per, b_total_cost, b_mass_per, b_total_mass) = b;

                            let orders = [
                                a_component.cmp(b_component),
                                a_count.cmp(b_count),
                                a_cost_per.cmp(b_cost_per),
                                a_total_cost.cmp(b_total_cost),
                                a_mass_per.partial_cmp(b_mass_per).unwrap_or(Ordering::Equal),
                                a_total_mass.partial_cmp(b_total_mass).unwrap_or(Ordering::Equal),
                            ];

                            orders[0].then(orders[1]).then(orders[2]).then(orders[3])
                        });

                        let sorted = create_signal(cx, sorted);
                        let vehicle_total_cost = create_memo(cx, || sorted.get().iter().map(|c| c.3.unwrap_or_default()).sum::<u32>());
                        let vehicle_total_mass = create_memo(cx, || sorted.get().iter().map(|c| c.5.unwrap_or_default()).sum::<f32>());

                        view! { cx,
                            p {
                                (format!("Total Cost: ${vehicle_total_cost}"))
                            }
                            p {
                                (format!("Total Mass: {vehicle_total_mass:.1}"))
                            }
                            table {
                                tr {
                                    th (
                                        title = "Click to Sort",
                                        on:click = |_| {
                                            let mut new_sorted = (*sorted.get()).clone();
                                            new_sorted.sort_by(|a, b| a.0.cmp(&b.0));
                                            sorted.set(new_sorted);
                                        }
                                    ) { "Component" }
                                    th (
                                        title = "Click to Sort",
                                        on:click = |_| {
                                            let mut new_sorted = (*sorted.get()).clone();
                                            new_sorted.sort_by(|a, b| a.1.cmp(&b.1).reverse());
                                            sorted.set(new_sorted);
                                        }
                                    ) { "Count" }
                                    th (
                                        title = "Click to Sort",
                                        on:click = |_| {
                                            let mut new_sorted = (*sorted.get()).clone();
                                            new_sorted.sort_by(|a, b| a.2.cmp(&b.2).reverse());
                                            sorted.set(new_sorted);
                                        }
                                    ) { "Cost Per" }
                                    th (
                                        title = "Click to Sort",
                                        on:click = |_| {
                                            let mut new_sorted = (*sorted.get()).clone();
                                            new_sorted.sort_by(|a, b| a.3.cmp(&b.3).reverse());
                                            sorted.set(new_sorted);
                                        }
                                    ) { "Cost Total" }
                                    th (
                                        title = "Click to Sort",
                                        on:click = |_| {
                                            let mut new_sorted = (*sorted.get()).clone();
                                            new_sorted.sort_by(|a, b| a.4.partial_cmp(&b.4).unwrap_or(Ordering::Equal).reverse());
                                            sorted.set(new_sorted);
                                        }
                                    ) { "Mass Per" }
                                    th (
                                        title = "Click to Sort",
                                        on:click = |_| {
                                            let mut new_sorted = (*sorted.get()).clone();
                                            new_sorted.sort_by(|a, b| a.5.partial_cmp(&b.5).unwrap_or(Ordering::Equal).reverse());
                                            sorted.set(new_sorted);
                                        }
                                    ) { "Mass Total" }
                                }
                                Indexed (
                                    iterable = sorted,
                                    view = move |cx, (component, count, cost_per, total_cost, mass_per, total_mass)| {
                                        let fract_cost = total_cost.map(|c| c as f32 / *vehicle_total_cost.get() as f32);
                                        let fract_mass = total_mass.map(|c| c / *vehicle_total_mass.get());

                                        view! { cx,
                                            tr {
                                                td { (component) }
                                                td { (count) }
                                                td { (cost_per.map(|c| format!("${c}")).unwrap_or_else(|| "Unknown".to_owned())) }
                                                td (style = format!("background-image: linear-gradient(to right, #c22 {0}%, transparent {0}%); background-repeat: no-repeat;", fract_cost.unwrap_or_default() * 100.0)) {
                                                    div (style = "float: left;") {
                                                        (total_cost.map(|c| format!("${c}")).unwrap_or_else(|| "Unknown".to_owned()))
                                                    }
                                                    div (style = "float: right; margin-left: 12px") {
                                                        ({
                                                            fract_cost.map(|f| {
                                                                let n = format!("{:.1}", f * 100.0);
                                                                let n = n.trim_end_matches('0').trim_end_matches('.');
                                                                format!("{n}%")
                                                            }).unwrap_or_default()
                                                        })
                                                    }
                                                }
                                                td { ({
                                                    if component == "Microprocessor" {
                                                        "(varies)".to_owned()
                                                    } else {
                                                        mass_per.map(|c| c.to_string()).unwrap_or_else(|| "Unknown".to_owned())
                                                    }
                                                }) }
                                                td (style = format!("background-image: linear-gradient(to right, #c22 {0}%, transparent {0}%); background-repeat: no-repeat;", fract_mass.unwrap_or_default() * 100.0)) {

                                                    div (style = "float: left;") {
                                                        (total_mass.map(|c| c.to_string()).unwrap_or_else(|| "Unknown".to_owned()))
                                                    }
                                                    div (style = "float: right; margin-left: 12px") {
                                                        ({
                                                            fract_mass.map(|f| {
                                                                let n = format!("{:.1}", f * 100.0);
                                                                let n = n.trim_end_matches('0').trim_end_matches('.');
                                                                format!("{n}%")
                                                            }).unwrap_or_default()
                                                        })
                                                    }
                                                }
                                            }
                                        }
                                    }
                                )
                            }
                        }
                    });

                    vehicle_components.unwrap_or_default()
                })
                // ul {
                //     (map)
                // }
                p (id = "grow-vertical") {}
                p {
                    (format!("Rom data built {}", state.rom_date.get().format("%B %-e %Y")))
                }
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope, _props: IndexPageState) -> View<SsrNode> {
    view! { cx,
        title { "Stormworks Vehicle Breakdown" }
        link(rel = "stylesheet", href = ".perseus/static/index.css") {}
        meta(name = "darkreader-lock") {}
    }
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> IndexPageState {
    let rom = std::env!("ROM_DIR");

    let mut definitions = HashMap::new();
    for entry in std::fs::read_dir(std::path::PathBuf::from(rom).join("data/definitions")).unwrap() {
        let entry = entry.unwrap();
        let id = entry
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        let text = std::fs::read_to_string(entry.path()).unwrap();

        let def: Definition = quick_xml::de::from_str(&text).unwrap();

        definitions.insert(id, def);
    }

    IndexPageState {
        rom_date: Utc::now(),
        status: "Drag a vehicle file".to_string(),
        definitions,
        vehicle: None,
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .head_with_state(head)
        .build()
}

#[derive(Debug, Default)]
struct ComponentTotal {
    pub count: u32,
    pub mass_mult: u32,
}