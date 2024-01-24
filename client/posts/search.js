import { redirectToPostPage} from "./posts.js";
import { getAllPosts } from "./postMethods.js";
export async function searchPosts() {
  const posts = await getAllPosts();
  const input = document.getElementById("search").value.toLowerCase();
  const sanitizedInput = purifySearch(input);
  const searchList = document.getElementById("search-list");

  searchList.innerHTML = "";

  posts.forEach((title) => {
    let searchValue = title.title.toLowerCase();
    if (searchValue.includes(sanitizedInput)) {
      const listItem = document.createElement("ul");
      listItem.textContent = title.title.toLowerCase();
  listItem.addEventListener("click", () => {
    console.log(title.dataset);
   redirectToPostPage(title._id.$oid);
 });
      searchList.appendChild(listItem);
      listItem.style.display = "block";
    }
  });

  if (searchList.children.length > 0 && sanitizedInput !== "") {
    searchList.style.display = "block";
  } else {
    searchList.style.display = "none";
  }
}


function purifySearch(input) {
  const sanitizedInput = DOMPurify.sanitize(input);
  if (sanitizedInput === "" ) {
    return null;
  }
  return sanitizedInput;
}
