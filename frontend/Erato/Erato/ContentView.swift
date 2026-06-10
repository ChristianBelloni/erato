//
//  ContentView.swift
//  Erato
//
//  Created by Christian Belloni on 10/06/26.
//

import SwiftUI
import BetterAuth

let userId = 21

struct ContentView: View {
  @EnvironmentObject private var authClient: BetterAuthClient

  var body: some View {
    if let user = authClient.session.data?.user {
      Text("Hello, \(user.name)")
    }

      if let session = authClient.session.data {
          if session.user.emailVerified {
              Button {
                  Task {
                      try await authClient.signOut()
                  }
              }
              label: {
                  Text("Sign out")
              }
          } else {
              Text("Check your email to verifify your account")
                  
              Button {
                  Task {
                      try await authClient.signOut()
                  }
              }
              label: {
                  Text("Sign out")
              }.task {
                  _ = await Task.detached(priority: .background) {
                      while !(await authClient.session.data?.user.emailVerified ?? false) {
                       try? await Task.sleep(for: .seconds(3))
                       await authClient.session.refreshSession()
                       }
                  }.result
              }
          }
    } else {
      Button {
        Task {
          try await authClient.signIn.email(with: .init(email: "user@example\(userId).com", password: "securepassword"))
        }
      }
      label: {
        Text("Sign in")
      }
      Button {
            Task {
                try await authClient.signUp.email(
                    with: .init(
                        email: "user@example\(userId).com",
                        password: "securepassword",
                        name: "userexample\(userId)"
                    )
                )
            }
        } label: {
            Text("Sign up")
        }
    }
  }
}

#Preview {
    ContentView()
}
