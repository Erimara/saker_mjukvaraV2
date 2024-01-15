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
            body: JSON.stringify({ email, password })
        });
        await response.json();
    } catch (error) {
        console.error("Error during login:", error);
    }
}