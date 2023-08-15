// #[actix: get]
struct Request();

use serde::{Serialize, Deserialize};

impl Request {
    fn body(&self) -> Option<&str> {
        todo!()
    }
}

struct HttpResponse {}

struct Error {}

fn handle_request(request: Request) -> Result<HttpResponse, Error> {
    match request.body().unwrap() {
        "DetachRequest" => {}

        "AttachRequest" => {}

        "CompleteWordsRequest" => {}
        "InitScriptRequest" => {}

        "ListClientsRequest" => {}

        "ListCommandsRequest" => {}

        "RemoveCommandsRequest" => {}

        "AddHelpPageRequest" => {}

        "PollUpdatesRequest" => {}

        "ParseCommandLineRequest" => {}

        "UpdateHelpPageRequest" => {}

        _ => {}

        // err = fmt.Errorf("unknown request %v", name)
        //     return
    }
    todo!()
}

// use {Serialize, Deserialize};

// use serde;

#[derive(Serialize, Deserialize)]
struct AttachRequest {
    shell: String,
    pid: u32,
    cod_binary_path: String,
}

#[derive(Serialize, Deserialize)]
struct AttachResponse {}


#[derive(Serialize, Deserialize)]
struct CompleteWordsRequest {
    // First word of the `Words` must be executable path.
    words: Vec<String>,
    cword: u32,
}

#[derive(Serialize, Deserialize)]
struct CompleteWordsResponse {
    completions: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct DetachRequest {
    pid: u32,
}

#[derive(Serialize, Deserialize)]
struct DetachResponse {}

#[derive(Serialize, Deserialize)]
struct InitScriptRequest {
    pid: u32,
}

#[derive(Serialize, Deserialize)]
struct InitScriptResponse {
    script: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct ShellAndPid {
    shell: String,
    pid: u32,
}

#[derive(Serialize, Deserialize)]
struct ListCommandsRequest {
    selectors: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct ListCommandsResponseItem {
    id: u64,

    // In rare cases Command might be empty.
    // Command: Option<datastore.Command >,
}

#[derive(Serialize, Deserialize)]
struct ListCommandsResponse {
    command_items: Vec<ListCommandsResponseItem>,
}

#[derive(Serialize, Deserialize)]
struct ListClientsRequest {}

#[derive(Serialize, Deserialize)]
struct ListClientsResponse {
    clients: Vec<ShellAndPid>,
}

#[derive(Serialize, Deserialize)]
struct RemoveCommandsRequest {
    help_page_ids: Vec<u64>,
}

#[derive(Serialize, Deserialize)]
struct RemoveCommandsResponse {}

#[derive(Serialize, Deserialize)]
struct AddHelpPageRequest {
    // Command: datastore.Command,
    // Policy: datastore.Policy,
}

#[derive(Serialize, Deserialize)]
struct AddHelpPageResponse {
    // HelpPage: datastore.HelpPage,
    // Status: datastore.AddHelpPageStatus,
}

#[derive(Serialize, Deserialize)]
struct ParseCommandLineRequest {
    pid: u32,
    command_line: String,
    dir: String,
    env: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct ParseCommandLineResponse {
    is_help_command: bool,
    // PolicyMode: datastore.Policy,
    args: Vec<String>,
    env: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct PollUpdatesRequest {
    pid: u32,
}

#[derive(Serialize, Deserialize)]
struct PollUpdatesResponse {
    script: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct UpdateHelpPageRequest {
    id: u64,
    // Command: datastore.Command,
}

#[derive(Serialize, Deserialize)]
struct UpdateHelpPageResponse {}

#[derive(Serialize, Deserialize)]
struct RemoteError {
    code: u32,
    message: String,
}