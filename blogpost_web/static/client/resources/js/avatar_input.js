
    document.getElementById('avatar_img_input').addEventListener('click', function(event) {


        var avatar_source=prompt("Avatar Url","");

        if (avatar_source!=null){
            var avatar = document.getElementById('avatar_img_input');


            if(!(/\.(png)$/i).test(avatar_source)){
                window.alert("Link must be to a .png file")
                avatar.src="client/resources/img/add.png"
                return;
            }

            avatar.onerror = () => {
                avatar.src="client/resources/img/add.png"
                window.alert("Invalid URL")
            }
            avatar.src = avatar_source;
        }

    });