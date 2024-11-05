
document.getElementById("post_form").addEventListener("submit", async (e) => {
    e.preventDefault();

    const formData = new FormData();

    let username_input = document.getElementById("username_input");
    let main_text_input = document.getElementById("main_text_input");
    let avatar_img_input = document.getElementById("avatar_img_input");
    let blog_img_input = document.getElementById("blog_img_btn");

    let username;
    let main_text;
    let avatar_img_src;
    let blog_img_src=null;
    let creation_date=new Date().toJSON().slice(0, 10);

    if (username_input.value === "") {
        alert("Please enter username");
        return;
    }
    username=username_input.value;


    if (main_text_input.value === "") {
        alert("Please enter main text");
        return;
    }
    main_text=main_text_input.value;


    if (!(avatar_img_input.src === "http://localhost:8080/client/resources/img/add.png")) {
        avatar_img_src=avatar_img_input.src;
    }

    if (blog_img_input.files[0]) {
        const file = blog_img_input.files[0];
        formData.append('file', file);
    }

    const postData = {
        username: username,
        main_text: main_text,
        creation_date:  creation_date,
        blog_img_src: blog_img_src,
        avatar_img_src: avatar_img_src
    };


    formData.append('dto', JSON.stringify(postData)); // Add JSON as a string

    fetch('http://localhost:8080/post', {
        method: 'POST',
        body: formData // The FormData instance takes care of setting the correct headers
    })
    .then(response => {
        if (!response.ok) {
            throw new Error('Network response was not ok: ' + response.statusText);
        }
        return response.text(); // Get the response text or use response.json() if the server returns JSON
    })
    .then(data => {
        console.log('Success:', data);
        location.reload();
    })
    .catch(error => {
        console.error('Error:', error);
    });




});