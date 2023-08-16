struct NoStarGlob(String);

struct StarGlob(String);

struct BaseNameGlob(String);


trait Selector {
    fn match_string(&self, s: String) -> bool;
}

fn CompileSelector() {

}