import { invoke } from "@tauri-apps/api/core";

const btnGetFiles = document.getElementById('btnGetFile') as HTMLElement;
const btnCreateFile = document.getElementById('btnCreateFile') as HTMLElement;
const list = document.getElementById('list') as HTMLElement;

const inputFileName = document.getElementById('fileName') as HTMLInputElement;
const fileContent = document.getElementById('fileContent') as HTMLElement;

window.addEventListener("DOMContentLoaded", () => {
  // 버튼에 클릭 이벤트 리스너 추가
  btnGetFiles.addEventListener('click', btnGetFilesClick);
  btnCreateFile.addEventListener('click', btnCreateFilesClick);
  
  // 버튼 클릭 시 실행될 함수 정의
  async function btnGetFilesClick() {
      if (btnGetFiles && list) {
        const result: string[] = await invoke("get_files", {
          path: "../files",
        });
  
        // ul 태크 초기화
        list.replaceChildren();
  
        result.forEach(element => {
          const li = document.createElement("li");

          li.addEventListener("click", async function () {
            fileContent.innerText = await readFile(element);
          });
          li.textContent = element;

          // a.href = url;
          list.appendChild(li);

          return li;
        });
      }
  }

  async function readFile(element: string) {
    let fileContentStr = "";

    const readResult: string[] = await invoke("read_file", {
      fileName: element,
    });

    readResult.forEach((readItem) => {
      fileContentStr += readItem + "\n";
    });

    return fileContentStr;
  }
  
  // 버튼 클릭 시 실행될 함수 정의
  async function btnCreateFilesClick() {
    if (btnGetFiles && list) {
      await invoke("create_file", {
        fileName: inputFileName.value,
      });
  
      btnGetFilesClick();
      inputFileName.value = "";
    }
  }
});
