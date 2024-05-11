use crate::views::layouts::MainLayout;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ViewArgs {
    redirect_url: String,
}

impl ViewArgs {
    pub fn new(redirect_url: impl Into<String>) -> ViewArgs {
        ViewArgs {
            redirect_url: redirect_url.into(),
        }
    }
}

#[function_component]
pub(crate) fn View(args: &ViewArgs) -> Html {
    html! {
      <MainLayout>

        <div class="container mx-auto bg-zinc-800 min-h-screen p-3">

            <h1 class="mb-8 text-4xl tracking-tight font-extrabold sm:text-5xl">{ format!("Developer Fake Logins") }</h1>

            <a class="bg-zinc-600 hover:bg-zinc-500 text-xl block mt-6 p-3" data-turbo="false" href={format!("{}?code=Leonardo", args.redirect_url)} >
                <img class="float-right h-80" src="/assets/leo.webp"/>
                <label class="text-xl" >{"Leonardo da Vinci"}</label>
                <br/>
                <span class="text-sm" >{"An Italian polymath of the High Renaissance who was active as a painter, draughtsman, engineer, scientist, theorist, sculptor, and architect. While his fame initially rested on his achievements as a painter, he has also become known for his notebooks, in which he made drawings and notes on a variety of subjects, including anatomy, astronomy, botany, cartography, painting, and paleontology. Leonardo is widely regarded to have been a genius who epitomized the Renaissance humanist ideal, and his collective works comprise a contribution to later generations of artists matched only by that of his younger contemporary Michelangelo"}</span>
                <div class="clear-both"></div>
            </a>


            <a class="bg-zinc-600 hover:bg-zinc-500 text-xl block mt-6 p-3" data-turbo="false" href={format!("{}?code=Michelangelo", args.redirect_url)} >
                <img class="float-right h-80" src="/assets/mic.webp"/>
                <label class="" >{"Michelangelo di Lodovico Buonarroti Simoni"}</label>
                <br/>
                <span class="text-sm" >{"An Italian sculptor, painter, architect, and poet of the High Renaissance. Born in the Republic of Florence, his work was inspired by models from classical antiquity and had a lasting influence on Western art. Michelangelo's creative abilities and mastery in a range of artistic arenas define him as an archetypal Renaissance man, along with his rival and elder contemporary, Leonardo da Vinci. Given the sheer volume of surviving correspondence, sketches, and reminiscences, Michelangelo is one of the best-documented artists of the 16th century. He was lauded by contemporary biographers as the most accomplished artist of his era"}</span>
                <div class="clear-both"></div>
            </a>


            <a class="bg-zinc-600 hover:bg-zinc-500 text-xl block mt-6 p-3" data-turbo="false" href={format!("{}?code=Donatello", args.redirect_url)} >
                <img class="float-right h-80" src="/assets/don.webp"/>
                <label class="" >{"Donato di Niccolò di Betto Bardi"}</label>
                <br/>
                <span class="text-sm" >{"an Italian sculptor of the Renaissance period. Born in Florence, he studied classical sculpture and used his knowledge to develop an Early Renaissance style of sculpture. He spent time in other cities, where he worked on commissions and taught others; his periods in Rome, Padua, and Siena introduced to other parts of Italy the techniques he had developed in the course of a long and productive career. His David was the first freestanding nude male sculpture since antiquity; like much of his work it was commissioned by the Medici family."}</span>
                <div class="clear-both"></div>
            </a>

            <a class="bg-zinc-600 hover:bg-zinc-500 text-xl block mt-6 p-3" data-turbo="false" href={format!("{}?code=Raffaello", args.redirect_url)} >
                <img class="float-right h-80" src="/assets/raff.webp"/>
                <label class="" >{"Raffaello Sanzio da Urbino"}</label>
                <br/>
                <span class="text-sm" >{"an Italian painter and architect of the High Renaissance. His work is admired for its clarity of form, ease of composition, and visual achievement of the Neoplatonic ideal of human grandeur. Together with Leonardo da Vinci and Michelangelo, he forms the traditional trinity of great masters of that period."}</span>
                <div class="clear-both"></div>
            </a>

        </div>


      </MainLayout>
    }
}
