
    document.getElementById('blog_img_btn').addEventListener('change', function(event) {

        const file = event.target.files[0];

        var img = document.getElementById('blog_img_input');
        img.src = URL.createObjectURL(file);
        img.style.display='block'

        var remove_btn = document.getElementById('remove_btn');
        remove_btn.style.display ="block"

        console.log(document.getElementById('blog_img_btn').value)


    });

    document.getElementById('remove_btn').addEventListener('click', function(event) {


        var img = document.getElementById('blog_img_input');
        img.removeAttribute('src');
        img.style.display='none'

        var remove_btn = document.getElementById('remove_btn');
        remove_btn.style.display ="none"

        console.log(document.getElementById('blog_img_btn').value)
        document.getElementById('blog_img_btn').value="";


    });

