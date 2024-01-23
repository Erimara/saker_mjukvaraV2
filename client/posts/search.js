export function searchPosts() {
  const input = document.getElementById("search").value.toLowerCase();
  const sanitizedInput = purifySearch(input);
  const postsList = document.getElementById("postsList");
  const searchList = document.getElementById("search-list");

  searchList.innerHTML = "";

  let titles = postsList.querySelectorAll("h4");

  titles.forEach((title) => {
    let searchValue = title.textContent.toLowerCase();

    if (searchValue.includes(sanitizedInput)) {
      const listItem = document.createElement("li");
      listItem.textContent = title.textContent;
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
