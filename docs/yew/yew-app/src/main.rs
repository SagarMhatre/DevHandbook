use yew::prelude::*;
use serde::Deserialize;
use gloo_net::http::Request;

#[derive(PartialEq, Deserialize,Clone)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[function_component(VideoDetails)]
fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct VideosTableProps {
    videos: Vec<Video>,
}

#[function_component(VideosTable)]
pub fn videos_table(VideosTableProps { videos }: &VideosTableProps) -> Html {

    let rows:Html =
    videos.iter().map(|video| html! {
        <tr>
        <td>{video.id}</td>
        <td>{format!("{}", video.speaker )}</td>
        <td>{format!("{}", video.title )}</td>
        </tr>
    }).collect::<Html>();

    let headers =[ "ID", "Speaker", "Title"];

    html! {
        <table>
        <tr>
        <th> {headers[0]}   </th><th> {headers[1]}  </th><th> {headers[2]}   </th>
        </tr>

        {rows}
        </table>
    }

}

#[function_component(App)]
fn app() -> Html {
    let my_videos = use_state(|| vec![]);
    {
        let videos = my_videos.clone();
        use_effect_with((), move |_| {
            let videos = videos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                //let fetched_videos: Vec<Video> = Request::get("https://yew.rs/tutorial/data.json")
                let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")        // To avoid CORS, we use proxy
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                videos.set(fetched_videos);
            });
            || ()
        });
    }
    

    let videos_paragraph = my_videos.iter().map(|video| html! {
    <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
    }).collect::<Html>();


    let selected_video = Video{
        id: 1,
        title: "Building and breaking things".to_string(),
        speaker: "John Doe".to_string(),
        url: "https://youtu.be/PsaFVLr8t4E".to_string()
    };

    let details =  html! {
                <VideoDetails video={selected_video} />
            };

    html! {
        <>
        <h1>{ "RustConf Explorer" }</h1>
        <div>
            <h3>{"Videos to watch"}</h3>
            <p>{ "John Doe: Building and breaking things" }</p>
            <p>{ "Jane Smith: The development process" }</p>
            <p>{ "Matt Miller: The Web 7.0" }</p>
            <p>{ "Tom Jerry: Mouseless development" }</p>
        </div>

        <div>
         {videos_paragraph}
        </div>


       
        <VideosTable videos ={(*my_videos).clone()} />
          
          {details}

    </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}