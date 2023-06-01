import { Component, OnInit } from "@angular/core";
import { invoke } from "@tauri-apps/api/tauri";
import Note from "src/models/Note";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styleUrls: ["./app.component.css"],
})
export class AppComponent implements OnInit {
  greetingMessage = "";
  filesDir = ""

  ngOnInit(): void {
    // invoke<string>("write_to_file", { text: "this is a test", fileName: "test2.txt" }).then(() => {
    //   console.log("done")
    // })

    invoke<string>("fetch_all_notes", {}).then((res: any) => {
      let notes = this.mapToType(res)
      console.log(notes)
    })

    invoke<string>("fetch_files_dir", {}).then((filesDir: string) => {
      console.log(filesDir)
      this.filesDir = filesDir
    }).then(() => {
      invoke<string>("fetch_note_content", { path: this.filesDir + "test_file.txt" }).then((content) => {
        console.log(content)
      })
    })
  }

  greet(event: SubmitEvent, name: string): void {
    event.preventDefault();

    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    invoke<string>("greet", { name }).then((text) => {
      this.greetingMessage = text;
    });
  }

  private mapToType(result: any[]): Note[] {
    let notes: Note[] = []
    result.forEach(res => {
      notes.push({ fileName: res.file_name, path: res.path })
    })

    return notes
  }
}
