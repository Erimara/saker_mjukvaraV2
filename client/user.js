
function emailRegexCheck(email) {
  const emailRegex = /^[a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,4}$/i;
  const checkEmail = emailRegex.test(email);
  console.log(email);
  if (checkEmail) {
    return email;
  } else return null;
}
function purifiedCredentials(email, password, option) {
const isValidEmail = emailRegexCheck(email);
  const sanitizedEmail = DOMPurify.sanitize(emailRegexCheck(email));
  const sanitizedPassword = DOMPurify.sanitize(password)
  if (!isValidEmail || sanitizedEmail === "" || sanitizedPassword === ""){
    if (option == "register"){
        document.getElementById("register").innerText =
          "Ineligible email or password.\nEmail may include, letters, digits, dots, underscores, or hyphens\n example@domain.io,";
        return null;
    } else if (option == "login"){
        document.getElementById("login").innerText =
          "Ineligible email or password"
        return null;
    }
    
  }
  return {sanitizedEmail, sanitizedPassword};
}
export async function registerUser(email, password) {
    const purifiedData = purifiedCredentials(email, password, "register");
    if(!purifiedData){
        return;
    }
  try {
    const response = await fetch("http://127.0.0.1:8081/register", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ 
        email:purifiedData.sanitizedEmail, 
        password:purifiedData.sanitizedPassword 
    }),
    });
    await response.json();
    if (response.ok) {
      document.getElementById("register").innerText =
        "Registration successfull";
    }
  } catch (error) {
    console.error("Error during registration:", error);
  }
}
export function sendData(e) {
       e.preventDefault();
       grecaptcha.enterprise.ready(async () => {
         const token = await grecaptcha.enterprise.execute(
           "6LeFqVgpAAAAANzbXhYcFL9_9bKs6L9VAY0p6aVy",
           { action: "LOGIN" }
         );
       });
     }
export async function login(email, password) {
    const purifiedData = purifiedCredentials(email, password, "login");
    if (!purifiedData) {
      return;
    }
  try {
    const response = await fetch("http://127.0.0.1:8081/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        email: purifiedData.sanitizedEmail,
        password: purifiedData.sanitizedPassword,
      }),
      credentials: "include",
    });
    let result = await response.json();
    if (response.ok) {
      setLoggedInUser(result);
    }
  } catch (error) {
    console.error("Error during login:", error);
  }
}

function setLoggedInUser(result) {
  let user = document.getElementById("user");
  let h4 = document.createElement("h4");
  h4.innerText = result;
  user.appendChild(h4);
  document.getElementById("login").innerText = "Login successfull";
  localStorage.setItem("loggedInUser", result);
  if (!result) {
    document.getElementById("login").innerText = "Wrong username or password";
  }
}
window.onload = function () {
  const username = localStorage.getItem("loggedInUser");
  if (username) {
    document.getElementById("sign-out").style.display = "block"
    setLoggedInUser(username);
  }
};

export async function logout() {
  try {
    const response = await fetch("http://127.0.0.1:8081/logout", {
      method: "DELETE",
      credentials: "include",
    });

    if (response.ok) {
      const result = await response.json();
      document.cookie = "user_id" + "=; HttpOnly; Max-Age=0";
      localStorage.clear("loggedInUser");
    } else {
      console.error("Error during logout");
    }
  } catch (error) {
    console.error("Error during logout:", error);
  }
}
