# Mass Assignment Vulnerabilities in Rust

Welcome to this Snyk Learn lesson on Mass Assignment vulnerabilities, demonstrated using Rust and the actix-web framework. This lesson will show you how this vulnerability can be exploited and how to prevent it using idiomatic Rust patterns.

📖 Lesson Summary

Mass Assignment is a vulnerability that occurs when a web framework automatically binds HTTP request parameters to a program's data models or objects. While this feature can save developers time, it becomes a security risk when the data model contains sensitive fields that a user should not be able to control directly. 


For example, a 

User model might contain fields like username and email, but also sensitive fields like role or isAdmin. An attacker could exploit mass assignment by adding these sensitive fields to their web request (e.g., sending 



{"role": "administrator"}), potentially granting themselves administrative privileges. 


In this lesson, we will:

Run a demo actix-web application that contains a vulnerable user creation endpoint.

Exploit the mass assignment vulnerability to create a user with elevated privileges.

Examine a secure endpoint that uses the Data Transfer Object (DTO) pattern to prevent the vulnerability.

Understand why the DTO pattern is an effective mitigation strategy in Rust.

⚙️ Setting Up the Demo Application
Prerequisites
You must have the Rust toolchain installed. If you don't, please install it from rustup.rs.

Step-by-Step Instructions
Save the Code:
Create a new project directory and save the Rust code provided in the lesson into a file named src/main.rs. Save the Cargo.toml content into a file named Cargo.toml.

Bash

mkdir mass-assignment
cd mass-assignment
mkdir src

### Now create main.rs and Cargo.toml with the provided content

Build the Application:
Navigate to the project's root directory in your terminal and use cargo to build the project. This will also download the required dependencies.

Bash

cargo build
Run the Application:
Once the build is complete, run the application.

Bash

cargo run
You should see a message indicating the server has started:
🚀 Server starting on http://127.0.0.1:8080

💥 Demonstrating the Vulnerability
Our application exposes a vulnerable endpoint at /vulnerable/user/create. This endpoint directly deserializes the incoming JSON into the User database model, which includes the sensitive role and organization fields.

Exploit Steps
Open a new terminal. The server you started in the previous step must remain running.

Craft a Malicious Request:
We will use curl to send a POST request to the vulnerable endpoint. The JSON payload will contain the standard fields (username, password, email) but also include the role and organization fields with malicious values.

Bash

curl -X POST http://127.0.0.1:8080/vulnerable/user/create \
-H "Content-Type: application/json" \
-d '{
  "username": "H4xx0r",
  "password": "H4xx0rP@$$w0rd",
  "email": "attacker@hax4hire.xyz",
  "role": "administrator",
  "organization": "cowmoo_industries"
}'
Analyze the Response:
The server will respond with the created user object. Notice that the role and organization have been set to the values we provided in the request. We have successfully elevated our privileges.

Expected Output:

JSON

{
  "username": "H4xx0r",
  "password": "H4xx0rP@$$w0rd",
  "email": "attacker@hax4hire.xyz",
  "role": "administrator",
  "organization": "cowmoo_industries"
}
In the terminal running the server, you will see a log confirming the creation of the compromised user:
[VULNERABLE] Created user: User { username: "H4xx0r", ..., role: "administrator", organization: "cowmoo_industries" }

🛡️ Demonstrating the Mitigation
The most effective way to prevent mass assignment in a statically-typed language like Rust is to use the Data Transfer Object (DTO) pattern. A DTO is a struct that represents the data expected from the client, containing 

only the fields they are allowed to set. 

Our application exposes a secure endpoint at /secure/user/create. This endpoint deserializes the request into a CreateUserDto, which only includes username, password, and email.

Mitigation Steps
Send the Same Malicious Request to the Secure Endpoint:
Now, let's send the exact same malicious payload to the /secure/user/create endpoint.

```bash
curl -X POST http://127.0.0.1:8080/secure/user/create \
-H "Content-Type: application/json" \
-d '{
  "username": "H4xx0r",
  "password": "H4xx0rP@$$w0rd",
  "email": "safe-user@hax4hire.xyz",
  "role": "administrator",
  "organization": "cowmoo_industries"
}'
```

Analyze the Secure Response:
Observe the response from the server. Even though we sent values for role and organization, they were ignored. The User object was created with secure, default values for these sensitive fields.

Expected Output:

JSON

{
  "username": "H4xx0r",
  "password": "H4xx0rP@$$w0rd",
  "email": "safe-user@hax4hire.xyz",
  "role": "user",
  "organization": "default_org"
}
The server log confirms that the handler set default values:
[SECURE] Created user: User { username: "H4xx0r", ..., role: "user", organization: "default_org" }

How the Fix Works
The secure handler, create_user_secure, has its argument typed as web::Json<CreateUserDto>. Because CreateUserDto does not contain role or organization fields, serde does not deserialize them from the input. The handler then manually constructs the full User object, pulling the safe fields from the DTO and setting the sensitive fields to hardcoded, secure defaults. This acts as a strict 

allow-list, which is the recommended mitigation strategy for mass assignment. 