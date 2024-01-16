export async function registerUser(email, password) {
    try {
        const response = await fetch("http://127.0.0.1:8080/register", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ email, password })
        });
        await response.json();
    } catch (error) {
        console.error("Error during registration:", error);
    }
}

export async function login(email, password) {
    try {
        const response = await fetch("http://127.0.0.1:8080/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ email, password }),
            credentials: 'include'
        });
        let result = await response.json();
        console.log(result)
    } catch (error) {
        console.error("Error during login:", error);
    }
}
export async function logout() {
    try {
        const response = await fetch("http://127.0.0.1:8080/logout", {
            method: "DELETE",
            credentials: 'include'
        });

        if (response.ok) {
            const result = await response.json();
            console.log(result);
            document.cookie = "user_id" + "=; HttpOnly; Max-Age=0";

        } else {
            console.error("Error during logout:", response.statusText);
        }
    } catch (error) {
        console.error("Error during logout:", error);
    }
}