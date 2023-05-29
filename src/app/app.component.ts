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

  ngOnInit(): void {
    invoke<string>("write_to_file", { text: "this is a test", fileName: "test2.txt" }).then(() => {
      console.log("done")
    })

    invoke<string>("fetch_all_notes", {}).then((res: any) => {
      let notes = this.mapToType(res)
      console.log(notes)
      console.log(notes[0].path)
      console.log(notes[1].fileName)
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
