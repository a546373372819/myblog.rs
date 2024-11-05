# MyBlog.rs

MyBlog.rs is a web application for posting and reading blogposts, implemented for JetBrains internship task

## Installation

Pull the git repository with following command:

```bash
git clone https://github.com/a546373372819/myblog.rs
```
While in the directory, build the docker image as such:
```bash
docker build -t myblog .
```
Next, run the docker image:
```bash
docker run -p 8080:8080 --rm --name myblog1 myblog
```
Finally, you can access the blog website with the following link:
https://localhost:8080/home
