use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <>
        // Hero Section
            <section class="hero is-primary">
                <div class="hero-body">
                    <div class="container">
                        <h1 class="title">
                            { "About Us" }
                        </h1>
                        <h2 class="subtitle">
                            { "Learn more about our mission and the team behind it." }
                        </h2>
                    </div>
                </div>
            </section>

            // Mission Section
            <section class="section">
                <div class="container">
                    <h3 class="title is-3">{ "Our Mission" }</h3>
                    <p>
                        { "Our mission is to empower businesses to achieve operational excellence and innovation through advanced statistical process control and cloud engineering solutions. By harnessing the power of data analytics and cloud technology, we aim to optimize processes, enhance performance, and drive sustainable growth. We are committed to delivering tailored, high-impact solutions that solve real-world challenges, ensuring our clients not only survive but thrive in the ever-evolving digital landscape. As a one-man team, we promise agility, personalized service, and a relentless pursuit of quality and excellence in all we do."
                        }
                    </p>
                </div>
            </section>

            // Team Section
            // <section class="section">
            //     <div class="container">
            //         <h3 class="title is-3">{ "Meet the Team" }</h3>
            //         <div class="columns is-multiline">
            //             // Example Team Member 1
            //             <div class="column is-4">
            //                 <div class="card">
            //                     <div class="card-image">
            //                         <figure class="image is-4by3">
            //                             <img src="path/to/team-member-1-photo" alt="Team member 1"/>
            //                         </figure>
            //                     </div>
            //                     <div class="card-content">
            //                         <div class="media">
            //                             <div class="media-content">
            //                                 <p class="title is-4">{ "Team Member 1" }</p>
            //                                 <p class="subtitle is-6">{ "@username1" }</p>
            //                             </div>
            //                         </div>
            //                         <div class="content">
            //                             { "A brief bio of the team member. What they do, their background, and any fun fact or quote you'd like to share." }
            //                         </div>
            //                     </div>
            //                 </div>
            //             </div>
            //             // Repeat for other team members as needed
            //         </div>
            //     </div>
            // </section>
        </>
    }
}
