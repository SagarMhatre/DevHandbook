use yew::prelude::*;

#[derive(PartialEq)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
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

    let myVideos = vec![
    Video {
        id: 1,
        title: "Building and breaking things 2 ".to_string(),
        speaker: "John Doe".to_string(),
        url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    },
    Video {
        id: 2,
        title: "The development process 2 ".to_string(),
        speaker: "Jane Smith".to_string(),
        url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    },
    Video {
        id: 3,
        title: "The Web 7.0 2 ".to_string(),
        speaker: "Matt Miller".to_string(),
        url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    },
    Video {
        id: 4,
        title: "Mouseless development 2 ".to_string(),
        speaker: "Tom Jerry".to_string(),
        url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    },
];

let videosPara = myVideos.iter().map(|video| html! {
   <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
}).collect::<Html>();



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
         {videosPara}
        </div>

       
        <VideosTable videos ={myVideos} />
          

    </>
    }
}

/*

*/

fn main() {
    yew::Renderer::<App>::new().render();
}