
use ev::{MouseEvent, SubmitEvent};
use html::Input;
use leptos::*;
use uuid::Uuid;

use gloo_timers::future::TimeoutFuture;
fn main() {
    mount_to_body(|| view! { <App/> })
}

// region: --- ProgressBar


#[component]
fn ProgressBar(
    // Marks this as an optional prop. It will default to the default
    // value of its type, i.e., 0.
    #[prop(default = 100)]
    /// The maximum value of the progress bar.
    max: u16,
    // Will run `.into()` on the value passed into the prop.
    #[prop(into)]
    // `Signal<T>` is a wrapper for several reactive types.
    // It can be helpful in component APIs like this, where we
    // might want to take any kind of reactive value
    /// How much progress should be displayed.
    progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress
            max={max}
            value=progress
        />
        <br/>
    }
}
// endregion: --- ProgressBar

// region: --- ControlledComponent


#[component]
fn ControlledComponent() -> impl IntoView {
    // create a signal to hold the value
    let (name, set_name) = create_signal("Controlled".to_string());

    view! {
        <input type="text"
            // fire an event whenever the input changes
            on:input=move |ev| {
                // event_target_value is a Leptos helper function
                // it functions the same way as event.target.value
                // in JavaScript, but smooths out some of the typecasting
                // necessary to make this work in Rust
                set_name.set(event_target_value(&ev));
            }

            // the `prop:` syntax lets you update a DOM property,
            // rather than an attribute.
            //
            // IMPORTANT: the `value` *attribute* only sets the
            // initial value, until you have made a change.
            // The `value` *property* sets the current value.
            // This is a quirk of the DOM; I didn't invent it.
            // Other frameworks gloss this over; I think it's
            // more important to give you access to the browser
            // as it really works.
            //
            // tl;dr: use prop:value for form inputs
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}
// endregion: --- ControlledComponent

// region: --- UncontrolledComponent


#[component]
fn UncontrolledComponent() -> impl IntoView {
    // import the type for <input>
    use leptos::html::Input;

    let (name, set_name) = create_signal("Uncontrolled".to_string());

    // we'll use a NodeRef to store a reference to the input element
    // this will be filled when the element is created
    let input_element: NodeRef<Input> = create_node_ref();

    // fires when the form `submit` event happens
    // this will store the value of the <input> in our signal
    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element.get()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> to exist")
            // `NodeRef` implements `Deref` for the DOM element type
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_name.set(value);
    };



    view! {
        <form on:submit=on_submit>
            <input type="text"
                // here, we use the `value` *attribute* to set only
                // the initial value, letting the browser maintain
                // the state after that
                value=name

                // store a reference to this input in `input_element`
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}
// endregion: --- UncontrolledComponent

// region: --- ErrorHandling

#[component]
fn ErrorHandling() -> impl IntoView {

    let (value, set_value) = create_signal(Ok(0));

    // when input changes, try to parse a number from the input
    let on_input = move |ev| set_value.set(event_target_value(&ev).parse::<i32>());

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or something that's not a number!)"
            <input type="number" on:input=on_input/>
            // If an `Err(_) had been rendered inside the <ErrorBoundary/>,
            // the fallback will be displayed. Otherwise, the children of the
            // <ErrorBoundary/> will be displayed.
            <ErrorBoundary
                // the fallback receives a signal containing current errors
                fallback=|errors| view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        // we can render a list of errors
                        // as strings, if we'd like
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                                .collect::<Vec<_>>()
                            }
                        </ul>
                    </div>
                }
            >
                <p>
                    "You entered "
                    // because `value` is `Result<i32, _>`,
                    // it will render the `i32` if it is `Ok`,
                    // and render nothing and trigger the error boundary
                    // if it is `Err`. It's a signal, so this will dynamically
                    // update when `value` changes
                    <strong>{value}</strong>
                </p>
            </ErrorBoundary>
        </label>
    }
}


// endregion: --- ErrorHandling


// region: --- ParentChild
#[derive(Copy, Clone)]
struct SmallcapsContext(WriteSignal<bool>);

#[component]

fn Parent_Child()-> impl IntoView {
    let (red, set_red) = create_signal(false);
    let (right, set_right) = create_signal(false);
    let (italics, set_italics) = create_signal(false);
    let (smallcaps, set_smallcaps) = create_signal(false);
    provide_context(SmallcapsContext(set_smallcaps));

    view! {
            <p
                // class: attributes take F: Fn() => bool, and these signals all implement Fn()
                class:red=red
                class:right=right
                class:italics=italics
                class:smallcaps=smallcaps
            >
                "Lorem ipsum sit dolor amet."
            </p>

            // Button A: pass the signal setter
            <ButtonA setter=set_red/>

            // Button B: pass a closure
            <ButtonB on_click=move |_| set_right.update(|value| *value = !*value)/>

            // Button B: use a regular event listener
            // setting an event listener on a component like this applies it
            // to each of the top-level elements the component returns
            <ButtonC on:click=move |_| set_italics.update(|value| *value = !*value)/>

            // Button D gets its setter from context rather than props
            <ButtonD/>
        
}


}

#[component]
pub fn ButtonA(
    /// Signal that will be toggled when the button is clicked.
    setter: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <button
            on:click=move |_| setter.update(|value| *value = !*value)
        >
            "Toggle Red"
        </button>
    }
}

/// Button B receives a closure
#[component]
pub fn ButtonB<F>(
    /// Callback that will be invoked when the button is clicked.
    on_click: F,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! {
        <button
            on:click=on_click
        >
            "Toggle Right"
        </button>
    }



    // just a note: in an ordinary function ButtonB could take on_click: impl Fn(MouseEvent) + 'static
    // and save you from typing out the generic
    // the component macro actually expands to define a
    //
    // struct ButtonBProps<F> where F: Fn(MouseEvent) + 'static {
    //   on_click: F
    // }
    //
    // this is what allows us to have named props in our component invocation,
    // instead of an ordered list of function arguments
    // if Rust ever had named function arguments we could drop this requirement
}

/// Button C is a dummy: it renders a button but doesn't handle
/// its click. Instead, the parent component adds an event listener.
#[component]
pub fn ButtonC() -> impl IntoView {
    view! {
        <button>
            "Toggle Italics"
        </button>
    }
}

/// Button D is very similar to Button A, but instead of passing the setter as a prop
/// we get it from the context
#[component]
pub fn ButtonD() -> impl IntoView {
    let setter = use_context::<SmallcapsContext>().unwrap().0;

    view! {
        <button
            on:click=move |_| setter.update(|value| *value = !*value)
        >
            "Toggle Small Caps"
        </button>
    }
}

// endregion: --- ParentChild

// region: --- CildrenComponent
#[component]
fn Cildren_Component()-> impl IntoView {
    let (items, _set_items) = create_signal(vec![0, 1, 2]);
    let render_prop = move || {
        // items.with(...) reacts to the value without cloning
        // by applying a function. Here, we pass the `len` method
        // on a `Vec<_>` directly
        let len = move || items.with(Vec::len);
        view! {
            <p>"Length: " {len}</p>
        }
    };

    view! {
        // This component just displays the two kinds of children,
        // embedding them in some other markup
        <TakesChildren
            // for component props, you can shorthand
            // `render_prop=render_prop` => `render_prop`
            // (this doesn't work for HTML element attributes)
            render_prop
        >
            // these look just like the children of an HTML element
            <p>"Here's a child."</p>
            <p>"Here's another child."</p>
        </TakesChildren>
        <hr/>
        // This component actually iterates over and wraps the children
        <WrapsChildren>
            <p>"Here's a child."</p>
            <p>"Here's another child."</p>
        </WrapsChildren>
    }
}


#[component]
pub fn TakesChildren<F, IV>(
    /// Takes a function (type F) that returns anything that can be
    /// converted into a View (type IV)
    render_prop: F,
    /// `children` takes the `Children` type
    /// this is an alias for `Box<dyn FnOnce() -> Fragment>`
    /// ... aren't you glad we named it `Children` instead?
    children: Children,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <h1><code>"<TakesChildren/>"</code></h1>
        <h2>"Render Prop"</h2>
        {render_prop()}
        <hr/>
        <h2>"Children"</h2>
        {children()}
    }
}

#[component]
pub fn WrapsChildren(children: Children) -> impl IntoView {
    // children() returns a `Fragment`, which has a
    // `nodes` field that contains a Vec<View>
    // this means we can iterate over the children
    // to create something new!
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! { <li>{child}</li> })
        .collect::<Vec<_>>();

    view! {
        <h1><code>"<WrapsChildren/>"</code></h1>
        // wrap our wrapped children in a UL
        <ul>{children}</ul>
    }
}

// endregion: --- CildrenComponent


// region: --- CreateEffect
#[derive(Copy, Clone)]
struct LogContext(RwSignal<Vec<String>>);

#[component]
fn CreateEffect()-> impl IntoView {
        // Just making a visible log here
    // You can ignore this...
    let log = create_rw_signal::<Vec<String>>(vec![]);
    let logged = move || log.get().join("\n");

    // the newtype pattern isn't *necessary* here but is a good practice
    // it avoids confusion with other possible future `RwSignal<Vec<String>>` contexts
    // and makes it easier to refer to it
    provide_context(LogContext(log));

    view! {
        <CreateAnEffect/>
        <pre>{logged}</pre>
    }
}

#[component]
fn CreateAnEffect() -> impl IntoView {
    let (first, set_first) = create_signal(String::new());
    let (last, set_last) = create_signal(String::new());
    let (use_last, set_use_last) = create_signal(true);

    // this will add the name to the log
    // any time one of the source signals changes
    create_effect(move |_| {
        log(if use_last.get() {
            with!(|first, last| format!("{first} {last}"))
        } else {
            first.get()
        })
    });

    view! {
        <h1>
            <code>"create_effect"</code>
            " Version"
        </h1>
        <form>
            <label>
                "First Name"
                <input
                    type="text"
                    name="first"
                    prop:value=first
                    on:change=move |ev| set_first.set(event_target_value(&ev))
                />
            </label>
            <label>
                "Last Name"
                <input
                    type="text"
                    name="last"
                    prop:value=last
                    on:change=move |ev| set_last.set(event_target_value(&ev))
                />
            </label>
            <label>
                "Show Last Name"
                <input
                    type="checkbox"
                    name="use_last"
                    prop:checked=use_last
                    on:change=move |ev| set_use_last.set(event_target_checked(&ev))
                />
            </label>
        </form>
    }
}

#[component]
fn ManualVersion() -> impl IntoView {
    let first = create_node_ref::<Input>();
    let last = create_node_ref::<Input>();
    let use_last = create_node_ref::<Input>();

    let mut prev_name = String::new();
    let on_change = move |_| {
        log("      listener");
        let first = first.get().unwrap();
        let last = last.get().unwrap();
        let use_last = use_last.get().unwrap();
        let this_one = if use_last.checked() {
            format!("{} {}", first.value(), last.value())
        } else {
            first.value()
        };

        if this_one != prev_name {
            log(&this_one);
            prev_name = this_one;
        }
    };

    view! {
        <h1>"Manual Version"</h1>
        <form on:change=on_change>
            <label>"First Name" <input type="text" name="first" node_ref=first/></label>
            <label>"Last Name" <input type="text" name="last" node_ref=last/></label>
            <label>
                "Show Last Name" <input type="checkbox" name="use_last" checked node_ref=use_last/>
            </label>
        </form>
    }
}

#[component]
fn EffectVsDerivedSignal() -> impl IntoView {
    let (my_value, set_my_value) = create_signal(String::new());
    // Don't do this.
    /*let (my_optional_value, set_optional_my_value) = create_signal(Option::<String>::None);

    create_effect(move |_| {
        if !my_value.get().is_empty() {
            set_optional_my_value(Some(my_value.get()));
        } else {
            set_optional_my_value(None);
        }
    });*/

    // Do this
    let my_optional_value =
        move || (!my_value.with(String::is_empty)).then(|| Some(my_value.get()));

    view! {
        <input prop:value=my_value on:input=move |ev| set_my_value.set(event_target_value(&ev))/>

        <p>
            <code>"my_optional_value"</code>
            " is "
            <code>
                <Show when=move || my_optional_value().is_some() fallback=|| view! { "None" }>
                    "Some(\""
                    {my_optional_value().unwrap()}
                    "\")"
                </Show>
            </code>
        </p>
    }
}


#[component]
pub fn Show<F, W, IV>(
    /// The components Show wraps
    children: Box<dyn Fn() -> Fragment>,
    /// A closure that returns a bool that determines whether this thing runs
    when: W,
    /// A closure that returns what gets rendered if the when statement is false
    fallback: F,
) -> impl IntoView
where
    W: Fn() -> bool + 'static,
    F: Fn() -> IV + 'static,
    IV: IntoView,
{
    let memoized_when = create_memo(move |_| when());

    move || match memoized_when.get() {
        true => children().into_view(),
        false => fallback().into_view(),
    }
}

fn log(msg: impl std::fmt::Display) {
    let log = use_context::<LogContext>().unwrap().0;
    log.update(|log| log.push(msg.to_string()));
}





// endregion: --- CreateEffect

// region: --- Asyncdataloading
async fn load_data(value: i32) -> i32 {
    // fake a one-second delay
    TimeoutFuture::new(1_000).await;
    if value==0 {
        value + 1
    }
    else {
        value * 10
    }
    
}


#[component]
fn AsyncDataLoading() -> impl IntoView {
    // this count is our synchronous, local state
    let (count, set_count) = create_signal(0);

    // create_resource takes two arguments after its scope
    let async_data = create_resource(
        // the first is the "source signal"
            move || count.get(),
        // the second is the loader
        // it takes the source signal's value as its argument
        // and does some async work
        |value| async move { load_data(value).await },
    );
    // whenever the source signal changes, the loader reloads

    // you can also create resources that only load once
    // just return the unit type () from the source signal
    // that doesn't depend on anything: we just load it once
    let stable = create_resource(|| (), |_| async move { load_data(1).await });

    // we can access the resource values with .read()
    // this will reactively return None before the Future has resolved
    // and update to Some(T) when it has resolved
    let async_result = move || {
        async_data
            .get()
            .map(|value| format!("Server returned {value:?}"))
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading...".into())
    };

    // the resource's loading() method gives us a
    // signal to indicate whether it's currently loading
    let loading = async_data.loading();
    let is_loading = move || if loading.get() { "Loading..." } else { "Idle." };

    view! {

        <h1>"AsyncDataLoading"</h1>
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me"
        </button>
        <p>
            <code>"stable"</code>": " {move || stable.get()}
        </p>
        <p>
            <code>"count"</code>": " {count} 
        </p>
        <p>
            <code>"async_value"</code>": "
            {async_result}
            <br/>
            {is_loading}
        </p>
    }
}

// endregion: --- Asyncdataloading

// region: --- Suspense
async fn important_api_call(name: String) -> String {
    TimeoutFuture::new(1_000).await;
    name.to_ascii_uppercase()
}

#[component]
fn Sus_pense() -> impl IntoView {
    let (name, set_name) = create_signal("Bill".to_string());

    // this will reload every time `name` changes
    let async_data = create_resource(move || name.get(), |name| async move { important_api_call(name).await });

    view! {
        <h1>"Suspense"</h1>
        <input
            on:input=move |ev| {
                set_name.set(event_target_value(&ev));
            }
            prop:value=name
        />
        <p><code>"name:"</code> {name}</p>
        <Suspense
            // the fallback will show whenever a resource
            // read "under" the suspense is loading
            fallback=move || view! { <p>"Loading..."</p> }>
            // the children will be rendered once initially,
            // and then whenever any resources has been resolved
            <p>
                "Your shouting name is "
                {move || async_data.get().unwrap_or("default".to_string())}
            </p>
        </Suspense>
    }
}

// endregion: --- Suspense


// region: --- Transition
async fn important_api_call2(id: usize) -> String {
    TimeoutFuture::new(1_000).await;
    match id {
        0 => "Alice",
        1 => "Bob",
        2 => "Carol",
        _ => "User not found",
    }
    .to_string()
}

#[component]
fn Trans_ition() -> impl IntoView {
    let (tab, set_tab) = create_signal(0);

    // this will reload every time `tab` changes
    let user_data = create_resource(move || tab.get(), |tab| async move { important_api_call2(tab).await });

    view! {
        <div class="buttons">
            <button
                on:click=move |_| set_tab.set(0)
                class:selected=move || tab.get() == 0
            >
                "Tab A"
            </button>
            <button
                on:click=move |_| set_tab.set(1)
                class:selected=move || tab.get() == 1
            >
                "Tab B"
            </button>
            <button
                on:click=move |_| set_tab.set(2)
                class:selected=move || tab.get() == 2
            >
                "Tab C"
            </button>
        </div>
        <Transition
            // the fallback will show initially
            // on subsequent reloads, the current child will
            // continue showing
            fallback=move || view! { <p>"Loading initial data..."</p> }
        >   
        {move || if user_data.loading().get() {
            view! {<p> "Hang on...  " {move || user_data.get()}</p>}
        } else {
            view! {<p> {move || user_data.get()}</p>}
        }}

        </Transition>
        
    }
}
// endregion: --- Transition

// region: --- Actions
async fn add_todo(text: &str) -> Uuid {
    _ = text;
    // fake a one-second delay
    TimeoutFuture::new(1_000).await;
    // pretend this is a post ID or something
    Uuid::new_v4()
}


#[component]
fn Actionss() -> impl IntoView {
    // an action takes an async function with single argument
    // it can be a simple type, a struct, or ()
    let add_todo = create_action(|input: &String| {
        // the input is a reference, but we need the Future to own it
        // this is important: we need to clone and move into the Future
        // so it has a 'static lifetime
        let input = input.to_owned();
        async move { add_todo(&input).await }
    });

    // actions provide a bunch of synchronous, reactive variables
    // that tell us different things about the state of the action
    let submitted = add_todo.input();
    let pending = add_todo.pending();
    let todo_id = add_todo.value();

    let input_ref = create_node_ref::<Input>();

    view! {
        <form
            on:submit=move |ev| {
                ev.prevent_default(); // don't reload the page...
                let input = input_ref.get().expect("input to exist");
                add_todo.dispatch(input.value());
            }
        >
            <label>
                "What do you need to do?"
                <input type="text"
                    node_ref=input_ref
                />
            </label>
            <button type="submit">"Add Todo"</button>
        </form>
        <p>{move || pending.get().then(|| "Loading...")}</p>
        <p>
            "Submitted: "
            <code>{move || format!("{:#?}", submitted.get())}</code>
        </p>
        <p>
            "Pending: "
            <code>{move || format!("{:#?}", pending.get())}</code>
        </p>
        <p>
            "Todo ID: "
            <code>{move || format!("{:#?}", todo_id.get())}</code>
        </p>
    }
}
// endregion: --- Actions

// region: --- App
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count.get() * 2;
    let values = vec![0,1,2];
    view! {
        <div>
        <h1> "controlled component"</h1>
        <ControlledComponent/>
        <h1> "uncontrolled component"</h1>
        <UncontrolledComponent/>
            <ul>
                {values.into_iter()
                    .map(|n| view! {<li>{n}</li>})
                    .collect::<Vec<_>>()
                }
            </ul>
            <button class="red"
                on:click=move |_| {
                    set_count.update(|n| *n += 1);
                }
            >
                "Click me: "
                {count}
            </button>
            <progress
            // static attributes work as in HTML
            max="50"

            // passing a function to an attribute
            // reactively sets that attribute
            // signals are functions, so `value=count` and `value=move || count.get()`
            // are interchangeable.
            value=count
        >
        </progress>
            // Conditionally render the message when count is 50
            {move || if count.get() >= 50 {
                set_count.update(|n| *n = 0);
                view! {
                    <h1>"You won"</h1>
                }
            } else {
                view! { <h1></h1> } // No rendering when count != 50
            }}

            <ProgressBar max=50 progress=count/>
            <ProgressBar max=50 progress=Signal::derive(double_count)/>
            <hr/>
            <ErrorHandling/>
            <hr/>
            <Parent_Child/>
            <Cildren_Component/>
            <hr/>
            <CreateEffect/>
            <hr/>
            <AsyncDataLoading/>
            <hr/>
            <Sus_pense/>
            <hr/>
            <Trans_ition/>
            <hr/>
            <Actionss/>
            <hr/>
        </div>
    }
}

// endregion: --- App
