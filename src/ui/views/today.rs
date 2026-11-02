use std::{collections::HashMap, sync::Arc};

use iocraft::prelude::*;

use crate::{
    common::ICONS,
    ids::ThingsId,
    store::{Task, ThingsStore},
    ui::components::{
        empty_text::EmptyText,
        task_group::{TaskGroup, TaskGroupHeader},
        tasks::{TaskList, TaskOptions},
    },
};

const LIST_INDENT: u32 = 2;

#[derive(Default)]
struct AreaGroup<'a> {
    tasks: Vec<&'a Task>,
}

#[derive(Default)]
struct GroupedSection<'a> {
    unscoped: Vec<&'a Task>,
    by_project: Vec<(ThingsId, Vec<&'a Task>)>,
    by_area: Vec<(ThingsId, AreaGroup<'a>)>,
}

fn header_text(items: &[Task]) -> String {
    let project_count = items.iter().filter(|task| task.is_project()).count();
    let task_count = items.iter().filter(|task| !task.is_project()).count();
    if project_count > 0 {
        let label = if project_count == 1 {
            "project"
        } else {
            "projects"
        };
        format!(
            "{} Today  ({} tasks, {} {})",
            ICONS.today, task_count, project_count, label
        )
    } else {
        format!("{} Today  ({} tasks)", ICONS.today, task_count)
    }
}

fn has_regular(items: &[Task]) -> bool {
    items.iter().any(|task| !task.evening)
}

fn has_evening(items: &[Task]) -> bool {
    items.iter().any(|task| task.evening)
}

fn ensure_area_group<'a>(
    grouped: &mut GroupedSection<'a>,
    area_pos: &mut HashMap<ThingsId, usize>,
    area_uuid: &ThingsId,
) -> usize {
    if let Some(i) = area_pos.get(area_uuid).copied() {
        return i;
    }
    let i = grouped.by_area.len();
    grouped
        .by_area
        .push((area_uuid.clone(), AreaGroup::default()));
    area_pos.insert(area_uuid.clone(), i);
    i
}

fn ensure_project_group<'a>(
    grouped: &mut GroupedSection<'a>,
    project_pos: &mut HashMap<ThingsId, usize>,
    project_uuid: &ThingsId,
) -> usize {
    if let Some(i) = project_pos.get(project_uuid).copied() {
        return i;
    }
    let i = grouped.by_project.len();
    grouped.by_project.push((project_uuid.clone(), Vec::new()));
    project_pos.insert(project_uuid.clone(), i);
    i
}

fn group_regular_items<'a>(items: &'a [Task], store: &ThingsStore) -> GroupedSection<'a> {
    let mut grouped = GroupedSection::default();
    let mut project_pos: HashMap<ThingsId, usize> = HashMap::new();
    let mut by_area_pos: HashMap<ThingsId, usize> = HashMap::new();

    for task in items.iter().filter(|task| !task.evening) {
        if task.is_project() {
            if let Some(area_uuid) = store.effective_area_uuid(task) {
                let area_idx = ensure_area_group(&mut grouped, &mut by_area_pos, &area_uuid);
                grouped.by_area[area_idx].1.tasks.push(task);
            } else {
                grouped.unscoped.push(task);
            }
            continue;
        }

        let project_uuid = store.effective_project_uuid(task);
        let area_uuid = store.effective_area_uuid(task);

        match (project_uuid, area_uuid) {
            (Some(project_uuid), _) => {
                let project_idx =
                    ensure_project_group(&mut grouped, &mut project_pos, &project_uuid);
                grouped.by_project[project_idx].1.push(task);
            }
            (None, Some(area_uuid)) => {
                let area_idx = ensure_area_group(&mut grouped, &mut by_area_pos, &area_uuid);
                grouped.by_area[area_idx].1.tasks.push(task);
            }
            (None, None) => grouped.unscoped.push(task),
        }
    }

    grouped
}

enum SidebarGroup<'g, 'a> {
    Project(&'g ThingsId, &'g Vec<&'a Task>),
    Area(&'g ThingsId, &'g Vec<&'a Task>),
}

/// Order groups the way the Things sidebar does: projects without an area
/// first, then each area followed by its projects.
fn sidebar_ordered_groups<'g, 'a>(
    grouped: &'g GroupedSection<'a>,
    store: &ThingsStore,
) -> Vec<SidebarGroup<'g, 'a>> {
    let area_index = |area_uuid: &ThingsId| {
        store
            .areas_by_uuid
            .get(area_uuid)
            .map_or(i32::MAX, |area| area.index)
    };

    let mut keyed: Vec<((u8, i32, u8, i32), SidebarGroup<'g, 'a>)> = Vec::new();
    for (project_uuid, tasks) in &grouped.by_project {
        let project = store.tasks_by_uuid.get(project_uuid);
        let project_index = project.map_or(i32::MAX, |p| p.index);
        let key = match project.and_then(|p| store.effective_area_uuid(p)) {
            Some(area_uuid) => (1, area_index(&area_uuid), 1, project_index),
            None => (0, 0, 0, project_index),
        };
        keyed.push((key, SidebarGroup::Project(project_uuid, tasks)));
    }
    for (area_uuid, area_group) in &grouped.by_area {
        let key = (1, area_index(area_uuid), 0, 0);
        keyed.push((key, SidebarGroup::Area(area_uuid, &area_group.tasks)));
    }

    // Stable sort keeps first-seen order for ties.
    keyed.sort_by_key(|(key, _)| *key);
    keyed.into_iter().map(|(_, group)| group).collect()
}

fn evening_items(items: &[Task]) -> Vec<&Task> {
    items.iter().filter(|task| task.evening).collect()
}

fn id_prefix_len(store: &ThingsStore, items: &[Task]) -> usize {
    let mut ids = items
        .iter()
        .map(|task| task.uuid.clone())
        .collect::<Vec<_>>();
    for task in items {
        if let Some(project_uuid) = store.effective_project_uuid(task) {
            ids.push(project_uuid);
        }
        if let Some(area_uuid) = store.effective_area_uuid(task) {
            ids.push(area_uuid);
        }
    }
    store.unique_prefix_length(&ids)
}

#[derive(Default, Props)]
pub struct TodayViewProps<'a> {
    pub items: Option<&'a Vec<Task>>,
    pub detailed: bool,
}

#[component]
pub fn TodayView<'a>(hooks: Hooks, props: &TodayViewProps<'a>) -> impl Into<AnyElement<'a>> {
    let store = hooks.use_context::<Arc<ThingsStore>>().clone();
    let Some(items) = props.items else {
        return element! { Text(content: "") }.into_any();
    };

    let content: AnyElement<'a> = {
        if items.is_empty() {
            element! { EmptyText(content: "No tasks for today.") }.into_any()
        } else {
            let prefix_len = id_prefix_len(store.as_ref(), items);
            let regular = group_regular_items(items, store.as_ref());
            let evening = evening_items(items);

            let regular_options = TaskOptions {
                detailed: props.detailed,
                show_project: false,
                show_area: false,
                show_today_markers: false,
                show_staged_today_marker: true,
            };
            let evening_options = TaskOptions {
                detailed: props.detailed,
                show_project: true,
                show_area: true,
                show_today_markers: false,
                show_staged_today_marker: true,
            };

            let mut regular_blocks: Vec<AnyElement<'a>> = Vec::new();

            if !regular.unscoped.is_empty() {
                regular_blocks.push(
                    element! {
                        TaskGroup(
                            header: None,
                            items: regular.unscoped.clone(),
                            id_prefix_len: prefix_len,
                            options: regular_options,
                            indent_under_header: 0u16,
                            hidden_count: 0usize,
                        )
                    }
                    .into_any(),
                );
            }

            for group in sidebar_ordered_groups(&regular, store.as_ref()) {
                let (header, tasks) = match group {
                    SidebarGroup::Project(project_uuid, tasks) => (
                        TaskGroupHeader::Project {
                            project_uuid: project_uuid.clone(),
                            title: store.resolve_project_title(project_uuid),
                            id_prefix_len: prefix_len,
                        },
                        tasks,
                    ),
                    SidebarGroup::Area(area_uuid, tasks) => (
                        TaskGroupHeader::Area {
                            area_uuid: area_uuid.clone(),
                            title: store.resolve_area_title(area_uuid),
                            id_prefix_len: prefix_len,
                        },
                        tasks,
                    ),
                };
                regular_blocks.push(
                    element! {
                        TaskGroup(
                            header: Some(header),
                            items: tasks.clone(),
                            id_prefix_len: prefix_len,
                            options: regular_options,
                            indent_under_header: 2u16,
                            hidden_count: 0usize,
                        )
                    }
                    .into_any(),
                );
            }

            element! {
                    View(flex_direction: FlexDirection::Column, gap: 1) {
                        Text(
                            content: header_text(items),
                            wrap: TextWrap::NoWrap,
                            color: Color::Yellow,
                            weight: Weight::Bold,
                        )

                        #(if has_regular(items) {
                            Some(element! {
                                View(flex_direction: FlexDirection::Column, padding_left: LIST_INDENT, gap: 1) {
                                    #(regular_blocks)
                                }
                            })
                        } else { None })

                        #(if has_evening(items) {
                            Some(element! {
                                View(flex_direction: FlexDirection::Column, gap: 1) {
                                    Text(
                                        content: format!("{} This Evening", ICONS.evening),
                                        wrap: TextWrap::NoWrap,
                                        color: Color::Blue,
                                        weight: Weight::Bold,
                                    )
                                    View(flex_direction: FlexDirection::Column, padding_left: LIST_INDENT) {
                                        TaskList(
                                            items: evening,
                                            id_prefix_len: prefix_len,
                                            options: evening_options,
                                        )
                                    }
                                }
                            })
                        } else { None })
                    }
                }
                .into_any()
        }
    };

    content
}
