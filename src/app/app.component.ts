import { Component, OnInit } from "@angular/core";
import { invoke } from "@tauri-apps/api/tauri";
import Note from "src/models/Note";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styleUrls: ["./app.component.css"],
})
export class AppComponent implements OnInit {
  filesDir = ""
  notes: Note[] = []

  ngOnInit(): void {
    invoke<string>("write_to_file", { text: "lfsdgf;dj", fileName: "new_file.txt" }).then(() => {
      console.log("done")
      this.fetchAllNotes()
    })

    this.fetchAllNotes();

    invoke<string>("fetch_files_dir", {}).then((filesDir: string) => {
      console.log(filesDir)
      this.filesDir = filesDir
    }).then(() => {
      invoke<string>("fetch_note_content", { path: this.filesDir + "test_file.txt" }).then((content) => {
        console.log(content)
      })
    })
  }

  fetchAllNotes() {
    invoke<string>("fetch_all_notes", {}).then((res: any) => {
      this.notes = this.mapToType(res)
      console.log(this.notes)
    })
  }

  private mapToType(result: any[]): Note[] {
    let notes: Note[] = []
    result.forEach(res => {
      notes.push({ fileName: res.file_name, path: res.path, text: res.text })
    })

    return notes
  }
}
