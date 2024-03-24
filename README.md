# Actix Web Application
#### *By: Matthew Holden*


### Summary

This project containerizes a Rust Actix web application using Docker. 

### Cargo Dependencies

We've first initialized a new binary-based Cargo project called actix_app. The only additional crate required as a dependancy is the `actic-web` crate.

![image](https://github.com/matthold86/actix_app/assets/114833075/5ab8f248-92e8-4017-9c2e-c1da64c9ae94)

### Main.RS

Next, we'll need to set up the HTTP route handlers for the Actix web server. This main.rs file defines a GET and POST route. However the POST route is only included for testing POST request functionality and is not used for this application. 

![image](https://github.com/matthold86/actix_app/assets/114833075/64dacb4e-4cc8-457a-bb51-a9bef0b0a8f6)

### Application Deployed Locally

We want to deploy the application locally first to see if the `#[actix_web::main]` macro is properly executing the async main function. We bound the function to port 8080 so that is where the app will be deployed.

![image](https://github.com/matthold86/actix_app/assets/114833075/b6ba883e-2612-417c-9d7b-5c2e6aac15a9)


![image](https://github.com/matthold86/actix_app/assets/114833075/22c961ff-e689-4cfb-86e9-e3157a84f3df)

### Create Docker Image

Finally, we want to containerize the web application and create a docker image so that the application can be run consistently across different environments. To do this, we'll run `docker build -t actix-app .` from the terminal. Below is a picture of the Dockerfile and a picture of the Docker image build details stored on the Docker Desktop application.

![image](https://github.com/matthold86/actix_app/assets/114833075/558df53d-d1cc-403c-a0e9-acbca9c8c98b)


![image](https://github.com/matthold86/actix_app/assets/114833075/38018a60-151a-45ca-a03f-0e3b2853fbcc)

