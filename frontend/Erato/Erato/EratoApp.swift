//
//  EratoApp.swift
//  Erato
//
//  Created by Christian Belloni on 10/06/26.
//

import SwiftUI
import BetterAuth


@main
struct EratoApp: App {
    @StateObject private var authClient = BetterAuthClient(
        baseURL: URL(string: "https://948f-45-12-248-222.ngrok-free.app")!,
        scheme: "erato://"
    )

    var body: some Scene {
      WindowGroup {
        ContentView()
          .environmentObject(authClient)
          .task {
            // Explicitly fetch the initial session.
            // future changes to the session will
            // be automatically updated
              await authClient.session.refreshSession()
          }
      }
    }
}
