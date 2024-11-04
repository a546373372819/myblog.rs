

    window.onload = async function () {
        const response = await fetch('http://localhost:8080/posts');

        if (!response.ok) {
            throw new Error(`HTTP error! Status: ${response.status}`);
        }



        // Parse the response as JSON
        const posts = await response.json();

        const container = document.getElementById('post_feed');

        // Clear the container before adding new posts
        container.innerHTML = '';

        // Iterate over the posts and create HTML elements for each post
        posts.forEach(post => {

            let username=post.username;
            let main_text=post.main_text;
            let creation_date=post.creation_date;
            let blog_img_src="";
            let avatar_img_src="#";
            let avatar_display="none";

            if(post.blog_img_src!=null){
                blog_img_src=post.blog_img_src
            }
            if(post.avatar_img_src!=null){
                avatar_img_src=post.avatar_img_src
                avatar_display="block"
            }

            const postDiv = document.createElement('div');
            postDiv.classList.add('post');

            postDiv.innerHTML = `
                    <img class="avatar_img" src=${avatar_img_src} style="display: ${avatar_display}" alt="avatar" >
                    <p class="username" >${username}</p>
                    <textarea class="main_text" style="background: transparent" >${main_text}</textarea>
                    <img class="blog_img" src="${blog_img_src}" />
                    <p id="date" style="color: #fffbef">${creation_date}</p>

            `;

            // Append the post div to the container
            container.appendChild(postDiv);
        });

        console.log(posts);
    };
