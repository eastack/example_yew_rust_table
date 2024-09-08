use std::collections::HashSet;
use yew::{Callback, classes, function_component, Html, html, TargetCast, use_reducer, use_state};
use serde::Serialize;
use web_sys::{console, HtmlInputElement, InputEvent};
use yew_hooks::use_set;
use yew_hooks::use_async;
use yew_custom_components::pagination::Pagination;
use yew_custom_components::table::{Options, Table};
use yew_custom_components::table::types::{ColumnBuilder, TableData};
// use plotly::common::Mode;
use plotly::{Plot, Scatter};
use yew::prelude::*;

#[derive(PartialEq, Clone, Serialize)]
pub struct XsCache {
    pub energy_values: Vec<Vec<f64>>,
    pub cross_section_values: Vec<Vec<f64>>,
    pub selected: Vec<bool>,
}

#[derive(Properties, PartialEq)]
pub struct PlotProps {
    pub cache: XsCache,
}

#[function_component(App)]
pub fn plot_component(props: &PlotProps) -> Html {
    let cache = &props.cache;

    let p = use_async::<_, _, ()>({
        let cache = cache.clone();

        async move {
            let id = "plot-div";
            let mut plot = Plot::new();

            for (i, (energy, cross_section)) in cache.energy_values.iter().zip(&cache.cross_section_values).enumerate() {
                if cache.selected[i] {
                    let trace = Scatter::new(energy.clone(), cross_section.clone())
                        // .mode(Mode::Markers)
                        .name(&format!("Scatter Plot {}", i));
                    plot.add_trace(trace);
                }
            }

            let layout = plotly::Layout::new().title("Displaying a Chart in Yew");
            plot.set_layout(layout);

            plotly::bindings::new_plot(id, &plot).await;
            Ok(())
        }
    });

    // Only on first render
    use_effect_with((), move |_| {
        p.run();
    });

    html! {
        <div id="plot-div"></div>
    }
}

fn get_values_by_id(id: i32) -> (Vec<f64>, Vec<f64>) {
    match id {
        1 => (vec![1.0, 1.1, 1.2], vec![10.0, 10.1, 10.2]),
        2 => (vec![2.0, 2.1, 2.2], vec![20.0, 20.1, 20.2]),
        3 => (vec![3.0, 3.1, 3.2], vec![30.0, 30.1, 30.2]),
        4 => (vec![4.0, 4.1, 4.2], vec![40.0, 40.1, 40.2]),
        5 => (vec![5.0, 5.1, 5.2], vec![50.0, 50.1, 50.2]),
        6 => (vec![6.0, 6.1, 6.2], vec![60.0, 60.1, 60.2]),
        7 => (vec![7.0, 7.1, 7.2], vec![70.0, 70.1, 70.2]),
        8 => (vec![8.0, 8.1, 8.2], vec![80.0, 80.1, 80.2]),
        9 => (vec![9.0, 9.1, 9.2], vec![90.0, 90.1, 90.2]),
        10 => (vec![10.0, 10.1, 10.2], vec![100.0, 100.1, 100.2]),
        11 => (vec![11.0, 11.1, 11.2], vec![110.0, 110.1, 110.2]),
        12 => (vec![12.0, 12.1, 12.2], vec![120.0, 120.1, 120.2]),
        13 => (vec![13.0, 13.1, 13.2], vec![130.0, 130.1, 130.2]),
        14 => (vec![14.0, 14.1, 14.2], vec![140.0, 140.1, 140.2]),
        15 => (vec![15.0, 15.1, 15.2], vec![150.0, 150.1, 150.2]),
        16 => (vec![16.0, 16.1, 16.2], vec![160.0, 160.1, 160.2]),
        17 => (vec![17.0, 17.1, 17.2], vec![170.0, 170.1, 170.2]),
        18 => (vec![18.0, 18.1, 18.2], vec![180.0, 180.1, 180.2]),
        19 => (vec![19.0, 19.1, 19.2], vec![190.0, 190.1, 190.2]),
        20 => (vec![20.0, 20.1, 20.2], vec![200.0, 200.1, 200.2]),
        21 => (vec![21.0, 21.1, 21.2], vec![210.0, 210.1, 210.2]),
        22 => (vec![22.0, 22.1, 22.2], vec![220.0, 220.1, 220.2]),
        23 => (vec![23.0, 23.1, 23.2], vec![230.0, 230.1, 230.2]),
        24 => (vec![24.0, 24.1, 24.2], vec![240.0, 240.1, 240.2]),
        25 => (vec![25.0, 25.1, 25.2], vec![250.0, 250.1, 250.2]),
        26 => (vec![26.0, 26.1, 26.2], vec![260.0, 260.1, 260.2]),
        27 => (vec![27.0, 27.1, 27.2], vec![270.0, 270.1, 270.2]),
        28 => (vec![28.0, 28.1, 28.2], vec![280.0, 280.1, 280.2]),
        29 => (vec![29.0, 29.1, 29.2], vec![290.0, 290.1, 290.2]),
        30 => (vec![30.0, 30.1, 30.2], vec![300.0, 300.1, 300.2]),
        _ => unreachable!(), // Default case for invalid ids
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    // Mock data holder
    let data = use_reducer(crate::types::mock_data::Data::default);
    let mock_data = (*data).clone();

    // console::log_1(&plot_html.clone().into());


    // Search term
    let search_term = use_state(|| None::<String>);
    let search = (*search_term).as_ref().cloned();

    let page=use_state(||0usize);
    let current_page=(*page).clone();

    // Sum data
    let selected_indexes = use_set(HashSet::<usize>::new());
    let selected = selected_indexes.current().clone();

    let sum = selected.len();

    // this needs updating with a callback
    // let mut cache_energy_values = Vec::new();
    // let mut cache_cross_section_values = Vec::new();
    // let mut cache_selected = Vec::new();

    // for &selected_id in selected.iter() {
    //     let (energy, cross_section) = get_values_by_id(selected_id as i32);
    //     cache_energy_values.push(energy);
    //     cache_cross_section_values.push(cross_section);
    //     cache_selected.push(true);

    //     // Print the selected ID to the console
    //     console::log_1(&selected_id.clone().into());
    // }

    // let cache = XsCache {
    //     energy_values:cache_energy_values,
    //     cross_section_values:cache_cross_section_values,
    //     selected:cache_selected,
    // };

    let cache = XsCache {
        energy_values: vec![
            vec![5.0, 5.1, 5.2],
            vec![3.0, 4.0]
        ],
        cross_section_values: vec![
            vec![50.0, 50.1, 50.2],
            vec![7.0, 8.0]
        ],
        selected: vec![true, true],
    };
    
    console::log_1(&serde_wasm_bindgen::to_value(&cache).unwrap());

    // Column definition
    let columns = vec![
        ColumnBuilder::new("select").orderable(true).short_name("Select").data_property("select").header_class("user-select-none").build(),
        ColumnBuilder::new("id").orderable(true).short_name("ID").data_property("id").header_class("user-select-none").build(),
        ColumnBuilder::new("name").orderable(true).short_name("Name").data_property("name").header_class("user-select-none").build(),
        ColumnBuilder::new("value").orderable(true).short_name("Value").data_property("value").header_class("user-select-none").build(),
    ];


    // Table options
    let options = Options {
        unordered_class: Some("fa-sort".to_string()),
        ascending_class: Some("fa-sort-up".to_string()),
        descending_class: Some("fa-sort-down".to_string()),
        orderable_classes: vec!["mx-1".to_string(), "fa-solid".to_string()],
    };


    
    // Handle sum
    let callback_sum = {
        let selected_indexes = selected_indexes.clone();
        Callback::from(move |index: usize| {
            if !selected_indexes.insert(index) {
                selected_indexes.remove(&index);
            }
        })
    };

    // Fill the table data structure with actual data
    let mut table_data = Vec::new();
    for (index, (id, name, value)) in mock_data.data.iter().enumerate() {
        table_data.push(TableLine {
            original_index: index,
            id: *id,
            name: name.clone(),
            value: *value,
            checked: selected.contains(&index),
            sum_callback: callback_sum.clone(),
        })
    }

    // Handle search input
    let oninput_search = {
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if input.value().is_empty() {
                search_term.set(None);
            } else {
                search_term.set(Some(input.value()));
            }
        })
    };

    let pagination_options = yew_custom_components::pagination::Options::default()
        .show_prev_next(true)
        .list_classes(vec!(String::from("pagination")))
        .item_classes(vec!(String::from("page-item")))
        .link_classes(vec!(String::from("page-link")))
        .active_item_classes(vec!(String::from("active")))
        .disabled_item_classes(vec!(String::from("disabled")));

    let handle_page = {
        let page = page.clone();
        Callback::from(move |id: usize| {
            page.set(id);
        })
    };


    
    html!(
        <>
            <h1>{"Minimal table Example"}</h1>
            <div class="flex-grow-1 p-2 input-group mb-2">
                <span class="input-group-text">
                    <i class="fas fa-search"></i>
                </span>
                <input class="form-control" type="text" id="search" placeholder="Search" oninput={oninput_search} />
            </div>
            <Table<TableLine> options={options.clone()} limit={Some(10)} page={current_page} search={search.clone()} classes={classes!("table", "table-hover")} columns={columns.clone()} data={table_data.clone()} orderable={true}/>
            <Pagination total={table_data.len()} limit={10} options={pagination_options} on_page={Some(handle_page)}/>
            <h5>{"Number selected"} <span class="badge text-bg-secondary">{sum}</span></h5>
            <div id="plot-div"></div>
            <App cache={cache} />
        </>
    )
} 




#[derive(Clone, Serialize, Debug, Default)]
struct TableLine {
    pub original_index: usize,
    pub id: i32,
    pub name: String,
    pub value: i64,
    pub checked: bool,
    #[serde(skip_serializing)]
    pub sum_callback: Callback<usize>,
}

impl PartialEq<Self> for TableLine {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value && self.checked == other.checked
    }
}

impl PartialOrd for TableLine {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl TableData for TableLine {
    fn get_field_as_html(&self, field_name: &str) -> yew_custom_components::table::error::Result<Html> {
        match field_name {
            "select" => Ok(html!( <input type="checkbox" checked={self.checked}
                onclick={
                let value = self.original_index;
                let handle_sum = self.sum_callback.clone();
                move |_| { handle_sum.emit(value); }
                } /> )
            ),
            "id" => Ok(html! { self.id }),
            "name" => Ok(html! { self.name.clone() }),
            "value" => Ok(html! { self.value }),
            _ => Ok(html! {}),
        }
    }

    fn get_field_as_value(&self, field_name: &str) -> yew_custom_components::table::error::Result<serde_value::Value> {
        match field_name {
            "id" => Ok(serde_value::Value::I32(self.id)),
            "name" => Ok(serde_value::Value::String(self.name.clone())),
            "value" => Ok(serde_value::Value::I64(self.value)),
            "select" => Ok(serde_value::Value::Bool(self.checked)),
            _ => Ok(serde_value::to_value(()).unwrap()),
        }
    }

    fn matches_search(&self, needle: Option<String>) -> bool {
        match needle {
            Some(needle) => self.name.to_lowercase().contains(&needle.to_lowercase()),
            None => true,
        }
    }
}
