// const DOMPurify = require("dompurify");
// function emailRegexCheck(email) {
//   const emailRegex = /^[a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,4}$/i;
//   const checkEmail = emailRegex.test(email);
//   if (checkEmail) {
//     return email;
//   } else return null;
// }
// function purifiedCredentials(email, password) {
//   const sanitizedEmail = DOMPurify.sanitize(emailRegexCheck(email));
//   const sanitizedPassword = DOMPurify.sanitize(password);
//   if (!sanitizedEmail || !sanitizedPassword) {
//     console.log("Invalid password or email");
//     return;
//   }
//   return { sanitizedEmail, sanitizedPassword };
// }
export async function registerUser(email, password) {
  try {
    const response = await fetch("http://127.0.0.1:8081/register", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ email, password }),
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


export async function login(email, password) {
  try {
    const response = await fetch("http://127.0.0.1:8081/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ email, password }),
      credentials: "include",
    });
    let result = await response.json();
    if (response.ok) {
      setLoggedInUser(result);
      hideSignOutButton(result);
    }
  } catch (error) {
    console.error("Error during login:", error);
  }
}

function setLoggedInUser(result) {
  let user = document.getElementById("user");
  let h4 = document.createElement("h4");
  h4.textContent = result;
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
      console.error("Error during logout:", response.statusText);
    }
  } catch (error) {
    console.error("Error during logout:", error);
  }
}
