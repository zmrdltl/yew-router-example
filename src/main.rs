use console_error_panic_hook;
use gloo_storage::{SessionStorage, Storage};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{Event, EventTarget, console, window};
use yew::prelude::*;
use yew_router::prelude::*;

// 메인 라우트 정의
#[derive(Clone, Routable, PartialEq, Serialize, Deserialize, Debug)]
enum MainRoute {
    #[at("/")]
    Home,
    #[at("/menu1")]
    Menu1,
    #[at("/menu1/*")]
    Menu1Submenu,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// 메뉴1의 하위 메뉴 라우트 정의
#[derive(Clone, Routable, PartialEq, Serialize, Deserialize, Debug)]
enum Menu1Route {
    #[at("/menu1")]
    Index,
    #[at("/menu1/submenu2")]
    Submenu2,
    #[at("/menu1/submenu3")]
    Submenu3,
    #[not_found]
    #[at("/menu1/404")]
    NotFound,
}

// 메뉴 상태를 저장하기 위한 키
const CURRENT_ROUTE_KEY: &str = "aice.current_route";
const CURRENT_SUBMENU_KEY: &str = "aice.current_submenu";

// 메인 컴포넌트
#[function_component(App)]
pub fn app() -> Html {
    // 페이지 로드시 새로고침 처리 초기화
    use_effect_with((), |_| {
        console::log_1(&"앱이 초기화됩니다.".into());
        initialize_page_refresh_handler();
        || ()
    });

    html! {
        <BrowserRouter>
            <div class="app-container">
                <AppStyle />
                <div class="header">
                    <h1>{"라우터 기반 새로고침, 앞, 뒤로가기"}</h1>
                </div>
                <div class="content">
                    <Switch<MainRoute> render={switch_main} />
                </div>
            </div>
        </BrowserRouter>
    }
}

// 메인 라우트 스위칭 함수
fn switch_main(route: MainRoute) -> Html {
    // 현재 메인 라우트 저장
    let _ = SessionStorage::set(CURRENT_ROUTE_KEY, route.clone());

    // 메인 라우트가 변경되면 submenu 상태도 적절하게 초기화
    if route == MainRoute::Home {
        let _ = SessionStorage::delete(CURRENT_SUBMENU_KEY);
    }

    match route {
        MainRoute::Home => html! {
            <div>
                <h2>{"홈 페이지"}</h2>
                <p>{"왼쪽 메뉴에서 항목을 선택하세요."}</p>
                <nav>
                    <Link<MainRoute> to={MainRoute::Menu1}>{ "메뉴1" }</Link<MainRoute>>
                </nav>
            </div>
        },
        MainRoute::Menu1 | MainRoute::Menu1Submenu => html! {
            <Menu1Component />
        },
        MainRoute::NotFound => html! {
            <div>
                <h2>{"404 - 페이지를 찾을 수 없습니다"}</h2>
                <Link<MainRoute> to={MainRoute::Home}>{ "홈으로 돌아가기" }</Link<MainRoute>>
            </div>
        },
    }
}

// 메뉴1 메인 컴포넌트
#[function_component(Menu1Component)]
fn menu1_component() -> Html {
    html! {
        <div class="menu-layout">
            <div class="sidebar">
                <h3>{"메뉴1"}</h3>
                <nav>
                    <ul>
                        <li><Link<Menu1Route> to={Menu1Route::Index}>{ "메뉴1 개요" }</Link<Menu1Route>></li>
                        <li><Link<Menu1Route> to={Menu1Route::Submenu2}>{ "하위메뉴2" }</Link<Menu1Route>></li>
                        <li><Link<Menu1Route> to={Menu1Route::Submenu3}>{ "하위메뉴3" }</Link<Menu1Route>></li>
                    </ul>
                </nav>
            </div>
            <div class="content">
                <Switch<Menu1Route> render={switch_menu1} />
            </div>
        </div>
    }
}

// 메뉴1 하위 라우트 스위칭 함수
fn switch_menu1(route: Menu1Route) -> Html {
    // 현재 하위 메뉴 상태 저장
    let _ = SessionStorage::set(CURRENT_SUBMENU_KEY, route.clone());

    // 메뉴1 라우트로 설정
    if route == Menu1Route::Index {
        let _ = SessionStorage::set(CURRENT_ROUTE_KEY, MainRoute::Menu1);
    } else {
        let _ = SessionStorage::set(CURRENT_ROUTE_KEY, MainRoute::Menu1Submenu);
    }

    match route {
        Menu1Route::Index => html! {
            <div>
                <h2>{"메뉴1 개요"}</h2>
                <p>{"이 페이지는 메뉴1의 메인 페이지입니다."}</p>
            </div>
        },
        Menu1Route::Submenu2 => html! {
            <div>
                <h2>{"하위메뉴2"}</h2>
                <p>{"하위메뉴2의 내용입니다. 새로고침하더라도 이 페이지를 기억합니다."}</p>
            </div>
        },
        Menu1Route::Submenu3 => html! {
            <div>
                <h2>{"하위메뉴3"}</h2>
                <p>{"하위메뉴3의 내용입니다. 새로고침하더라도 이 페이지를 기억합니다."}</p>
            </div>
        },
        Menu1Route::NotFound => html! {
            <Redirect<MainRoute> to={MainRoute::NotFound} />
        },
    }
}

// 페이지 초기 로드 및 새로고침 처리를 위한 함수
fn initialize_page_refresh_handler() {
    // 페이지 로드 시 이전 경로 복원
    restore_previous_route();

    // 페이지 종료 시 현재 경로 저장
    let window = window().expect("window 객체를 가져올 수 없습니다");
    let window_clone = window.clone();

    // beforeunload 이벤트 처리
    let callback = Closure::wrap(Box::new(move || {
        console::log_1(&"페이지를 떠납니다. 현재 상태를 저장합니다.".into());
        save_current_route(&window_clone);
        // 페이지 새로고침 처리를 위해 문자열 반환
        JsValue::from_str("")
    }) as Box<dyn FnMut() -> JsValue>);

    window.set_onbeforeunload(Some(callback.as_ref().unchecked_ref()));
    callback.forget(); // 메모리 누수 방지

    // popstate 이벤트도 처리 (브라우저 뒤로/앞으로 버튼 등)
    let window_clone2 = window.clone();
    let popstate_callback = Closure::wrap(Box::new(move |_: web_sys::Event| {
        console::log_1(&"popstate 이벤트 발생. 상태를 저장합니다.".into());
        save_current_route(&window_clone2);
    }) as Box<dyn FnMut(web_sys::Event)>);

    window
        .add_event_listener_with_callback("popstate", popstate_callback.as_ref().unchecked_ref())
        .expect("이벤트 리스너 등록 실패");

    popstate_callback.forget();
}

fn save_current_route(window: &web_sys::Window) {
    if let Some(location) = window.location().pathname().ok() {
        if location.starts_with("/menu1/submenu2") {
            let _ = SessionStorage::set(CURRENT_SUBMENU_KEY, Menu1Route::Submenu2);
            let _ = SessionStorage::set(CURRENT_ROUTE_KEY, MainRoute::Menu1Submenu);
        } else if location.starts_with("/menu1/submenu3") {
            let _ = SessionStorage::set(CURRENT_SUBMENU_KEY, Menu1Route::Submenu3);
            let _ = SessionStorage::set(CURRENT_ROUTE_KEY, MainRoute::Menu1Submenu);
        } else if location == "/menu1" {
            let _ = SessionStorage::set(CURRENT_SUBMENU_KEY, Menu1Route::Index);
            let _ = SessionStorage::set(CURRENT_ROUTE_KEY, MainRoute::Menu1);
        } else if location == "/" {
            let _ = SessionStorage::set(CURRENT_ROUTE_KEY, MainRoute::Home);
            let _ = SessionStorage::delete(CURRENT_SUBMENU_KEY);
        }
    }
}

fn restore_previous_route() {
    let window = window().expect("window 객체를 가져올 수 없습니다");
    let location = window.location();
    let pathname = location.pathname().unwrap_or_default();

    console::log_1(&format!("현재 경로: {}", pathname).into());

    if pathname == "/" {
        if let Ok(submenu) = SessionStorage::get::<Menu1Route>(CURRENT_SUBMENU_KEY) {
            match submenu {
                Menu1Route::Submenu2 => {
                    console::log_1(&"하위메뉴2로 복원합니다.".into());
                    let _ = location.set_pathname("/menu1/submenu2");
                    return;
                }
                Menu1Route::Submenu3 => {
                    console::log_1(&"하위메뉴3으로 복원합니다.".into());
                    let _ = location.set_pathname("/menu1/submenu3");
                    return;
                }
                Menu1Route::Index => {
                    console::log_1(&"메뉴1 메인으로 복원합니다.".into());
                    let _ = location.set_pathname("/menu1");
                    return;
                }
                _ => {}
            }
        } else if let Ok(main_route) = SessionStorage::get::<MainRoute>(CURRENT_ROUTE_KEY) {
            match main_route {
                MainRoute::Menu1 => {
                    console::log_1(&"메뉴1로 복원합니다.".into());
                    let _ = location.set_pathname("/menu1");
                    return;
                }
                MainRoute::Menu1Submenu => {
                    console::log_1(&"메뉴1로 복원합니다 (하위메뉴 정보 없음).".into());
                    let _ = location.set_pathname("/menu1");
                    return;
                }
                _ => {
                    // 홈으로 이동하는 경우는 이미 / 경로이므로 별도 처리 불필요
                }
            }
        }
    } else {
        // URL 경로에 따라 SessionStorage 상태 동기화
        if pathname.starts_with("/menu1/submenu2") {
            let _ = SessionStorage::set(CURRENT_SUBMENU_KEY, Menu1Route::Submenu2);
            let _ = SessionStorage::set(CURRENT_ROUTE_KEY, MainRoute::Menu1Submenu);
        } else if pathname.starts_with("/menu1/submenu3") {
            let _ = SessionStorage::set(CURRENT_SUBMENU_KEY, Menu1Route::Submenu3);
            let _ = SessionStorage::set(CURRENT_ROUTE_KEY, MainRoute::Menu1Submenu);
        } else if pathname == "/menu1" {
            let _ = SessionStorage::set(CURRENT_SUBMENU_KEY, Menu1Route::Index);
            let _ = SessionStorage::set(CURRENT_ROUTE_KEY, MainRoute::Menu1);
        } else if pathname == "/" {
            let _ = SessionStorage::set(CURRENT_ROUTE_KEY, MainRoute::Home);
            let _ = SessionStorage::delete(CURRENT_SUBMENU_KEY);
        }
    }
}

#[function_component(AppStyle)]
fn app_style() -> Html {
    html! {
        <style>{"
            .app-container {
                display: flex;
                flex-direction: column;
                height: 100vh;
                font-family: Arial, sans-serif;
            }
            .header {
                background-color: #2F333A;
                color: white;
                padding: 1rem;
                box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
            }
            .content {
                flex: 1;
                padding: 1rem;
            }
            .menu-layout {
                display: flex;
                height: 100%;
            }
            .sidebar {
                width: 200px;
                background-color: #f5f5f5;
                padding: 1rem;
                border-right: 1px solid #ddd;
            }
            .sidebar ul {
                list-style: none;
                padding: 0;
            }
            .sidebar li {
                margin-bottom: 0.5rem;
            }
            .sidebar a {
                display: block;
                padding: 0.5rem;
                color: #333;
                text-decoration: none;
                border-radius: 4px;
            }
            .sidebar a:hover {
                background-color: #e0e0e0;
            }
            .content {
                flex: 1;
                padding: 1rem;
            }
        "}</style>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    console::log_1(&"애플리케이션이 시작됩니다.".into());

    if let Ok(main_route) = SessionStorage::get::<MainRoute>(CURRENT_ROUTE_KEY) {
        console::log_1(&format!("저장된 메인 라우트: {:?}", &main_route).into());
    }

    if let Ok(submenu) = SessionStorage::get::<Menu1Route>(CURRENT_SUBMENU_KEY) {
        console::log_1(&format!("저장된 서브메뉴 라우트: {:?}", &submenu).into());
    }

    yew::Renderer::<App>::new().render();
}
