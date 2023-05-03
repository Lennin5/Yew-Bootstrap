use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {

  html! {
    <div>
        <button type="button" class="btn btn-pink">{"Primary"}</button>       

        <div class="container mb-5 border border-lila rounded-top-right-radius">
          <div class="row d-flex justify-content-center">
            <div class="col-12 col-md-6 col-lg-4">
              <div class="card">
                <img src="https://www.muycomputer.com/wp-content/uploads/2021/06/Monterey-1.jpg" class="card-img-top w-100" alt="..." />
                <div class="card-body">
                  <h5 class="card-title">{"Card titlwe"}</h5>
                  <p class="card-text">{"Some quick example text to build on the card title and make up the bulk of the card's content."}</p>
                  <a href="#" class="btn btn-primary">{"Go somewhere"}</a>
                </div>
              </div>              
            </div>
          </div>
        </div>

        <button type="button" class="btn btn-primary ms-50" data-bs-toggle="modal" data-bs-target="#exampleModal">
          {"Launch demo modal"}
        </button>

        <div class="modal fade" id="exampleModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
          <div class="modal-dialog">
            <div class="modal-content">
              <div class="modal-header">
                <h5 class="modal-title" id="exampleModalLabel">{"Modal title"}</h5>
                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
              </div>
              <div class="modal-body">
                <p>{"Modal body text goes here."}</p>
              </div>
              <div class="modal-footer">
                <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Close"}</button>
                <button type="button" class="btn btn-primary">{"Save changes"}</button>
              </div>
            </div>
          </div>
        </div>        

    </div>
  }
}

fn main() {
    yew::Renderer::<App>::new().render();
}