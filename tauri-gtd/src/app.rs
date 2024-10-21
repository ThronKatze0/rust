use leptos::*;
use leptos_icons::*;
use leptos_mview::mview;

enum SelectedTodoRoute {
    Inbox,
    Next,
    Later,
    Waiting,
    Scheduled,
    Someday,
    Focus,
    Projects(String),
    Refernces(String),
    Tags(String)
}

#[component]
pub fn App() -> impl IntoView {
    let (route, set_route) = create_signal(SelectedTodoRoute::Next);

    mview! {
        div class="flex flex-row" {
            ul class="menu bg-base-200 rounded-box w-36 h-full" {
                li class="menu-title"("CAPTURE")
                li { a { Icon icon={icondata::AiInboxOutlined}() "Inbox"} }
                li {}
                li class="menu-title"("ACTIONS")
                li { a { Icon icon={icondata::BiRightArrowCircleRegular}() "Next"} }
                li { a { Icon icon={icondata::AiPushpinOutlined}() "Later"} }
                li { a { Icon icon={icondata::BsCupHot}() "Waiting"} }
                li { a { Icon icon={icondata::AiScheduleOutlined}() "Scheduled"} }
                li { a { Icon icon={icondata::BsSunset}() "Someday"} }
                li {}
                li { a { Icon icon={icondata::AiStarOutlined}() "Focus"} }
                li {}
            }
            input()
        }
    }
}
