import { Component, OnInit } from "@angular/core";
import { invoke } from "@tauri-apps/api/tauri";
import Note from "src/models/Note";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styleUrls: ["./app.component.css"],
})
export class AppComponent implements OnInit {
  notes: Note[] = []

  ngOnInit(): void {
    this.fetchAllNotes();
  }

  private fetchAllNotes() {
    invoke<string>("fetch_all_notes_from_cache", {}).then((res: any) => {
      this.notes = this.mapToType(res)
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
