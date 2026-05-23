import { T as defineStore, a4 as ref, K as api, S as defineComponent, ab as useAppStore, a1 as onMounted, P as createElementBlock, M as createBaseVNode, R as createVNode, ai as withCtx, r as VIcon, e as VBtn, z as VRow, aa as unref, N as createBlock, O as createCommentVNode, o as VDialog, a2 as openBlock, Q as createTextVNode, F as Fragment, a5 as renderList, f as VCard, h as VCardItem, k as VCardTitle, a8 as toDisplayString, i as VCardSubtitle, j as VCardText, m as VChip, g as VCardActions, C as VSpacer, n as VCol, H as VTextField, A as VSelect, B as VSkeletonLoader, _ as _export_sfc } from "./main-gWZPyuWK.js";
const useMcpStore = defineStore("mcp", () => {
  const servers = ref([]);
  const loading = ref(false);
  const testingServer = ref(null);
  async function fetchServers() {
    loading.value = true;
    try {
      servers.value = await api.mcpList();
    } catch (e) {
      console.error("Failed to fetch MCP servers:", e);
      servers.value = [];
    } finally {
      loading.value = false;
    }
  }
  async function addServer(name, transport, command, args = [], env = {}, url, scope = "local") {
    const result = await api.mcpAdd(name, transport, command, args, env, url, scope);
    await fetchServers();
    return result;
  }
  async function removeServer(name) {
    await api.mcpRemove(name);
    await fetchServers();
  }
  async function testConnection(name) {
    testingServer.value = name;
    try {
      const result = await api.mcpTestConnection(name);
      return result;
    } finally {
      testingServer.value = null;
    }
  }
  async function importFromClaudeDesktop(scope = "local") {
    const result = await api.mcpAddFromClaudeDesktop(scope);
    await fetchServers();
    return result;
  }
  function isServerConnected(server) {
    var _a;
    return ((_a = server.status) == null ? void 0 : _a.running) ?? false;
  }
  return {
    servers,
    loading,
    testingServer,
    fetchServers,
    addServer,
    removeServer,
    testConnection,
    importFromClaudeDesktop,
    isServerConnected
  };
});
const _hoisted_1 = { class: "d-flex align-center justify-space-between mb-6" };
const _hoisted_2 = { class: "d-flex align-center" };
const _hoisted_3 = { class: "d-flex gap-3" };
const _hoisted_4 = { class: "d-flex align-center gap-4" };
const _hoisted_5 = {
  key: 0,
  class: "mt-3"
};
const _hoisted_6 = {
  key: 1,
  class: "mt-2"
};
const _hoisted_7 = { class: "text-caption text-error" };
const _hoisted_8 = {
  key: 1,
  class: "text-center py-12"
};
const _sfc_main = /* @__PURE__ */ defineComponent({
  __name: "McpPage",
  setup(__props) {
    const mcpStore = useMcpStore();
    const appStore = useAppStore();
    const showAddDialog = ref(false);
    const removeDialog = ref(false);
    const removeTarget = ref(null);
    const importing = ref(false);
    const addForm = ref({
      name: "",
      transport: "stdio",
      command: "",
      argsText: "",
      url: "",
      scope: "local"
    });
    function confirmRemove(server) {
      removeTarget.value = server;
      removeDialog.value = true;
    }
    async function handleRemove() {
      if (!removeTarget.value) return;
      try {
        await mcpStore.removeServer(removeTarget.value.name);
        appStore.showMessage("Server removed", "success");
      } catch {
        appStore.showMessage("Failed to remove server", "error");
      }
      removeDialog.value = false;
    }
    async function addServer() {
      try {
        await mcpStore.addServer(
          addForm.value.name,
          addForm.value.transport,
          addForm.value.command || void 0,
          addForm.value.argsText ? addForm.value.argsText.split(",").map((s) => s.trim()) : [],
          {},
          addForm.value.url || void 0,
          addForm.value.scope
        );
        appStore.showMessage("Server added", "success");
        showAddDialog.value = false;
        addForm.value = { name: "", transport: "stdio", command: "", argsText: "", url: "", scope: "local" };
      } catch {
        appStore.showMessage("Failed to add server", "error");
      }
    }
    async function testConnection(name) {
      try {
        const result = await mcpStore.testConnection(name);
        appStore.showMessage(
          result || "Connection test complete",
          "success"
        );
      } catch {
        appStore.showMessage("Connection failed", "error");
      }
    }
    async function importFromDesktop() {
      importing.value = true;
      try {
        const result = await mcpStore.importFromClaudeDesktop();
        appStore.showMessage(`Imported ${result.imported_count} server(s)`, "success");
      } catch {
        appStore.showMessage("Failed to import from Claude Desktop", "error");
      } finally {
        importing.value = false;
      }
    }
    onMounted(() => {
      console.log("McpPage onMounted");
      mcpStore.fetchServers();
    });
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("div", null, [
        createBaseVNode("div", _hoisted_1, [
          createBaseVNode("div", _hoisted_2, [
            createVNode(VIcon, {
              size: "32",
              color: "primary",
              class: "mr-3"
            }, {
              default: withCtx(() => [..._cache[12] || (_cache[12] = [
                createTextVNode("mdi-server-network", -1)
              ])]),
              _: 1
            }),
            _cache[13] || (_cache[13] = createBaseVNode("h1", { class: "text-h4 font-weight-bold" }, "MCP Servers", -1))
          ]),
          createBaseVNode("div", _hoisted_3, [
            createVNode(VBtn, {
              variant: "tonal",
              "prepend-icon": "mdi-cloud-download",
              onClick: importFromDesktop,
              loading: importing.value
            }, {
              default: withCtx(() => [..._cache[14] || (_cache[14] = [
                createTextVNode(" Import from Claude Desktop ", -1)
              ])]),
              _: 1
            }, 8, ["loading"]),
            createVNode(VBtn, {
              color: "primary",
              "prepend-icon": "mdi-plus",
              onClick: _cache[0] || (_cache[0] = ($event) => showAddDialog.value = true)
            }, {
              default: withCtx(() => [..._cache[15] || (_cache[15] = [
                createTextVNode(" Add Server ", -1)
              ])]),
              _: 1
            })
          ])
        ]),
        createVNode(VRow, null, {
          default: withCtx(() => [
            (openBlock(true), createElementBlock(Fragment, null, renderList(unref(mcpStore).servers, (server) => {
              return openBlock(), createBlock(VCol, {
                cols: "12",
                md: "6",
                key: server.name
              }, {
                default: withCtx(() => [
                  createVNode(VCard, { class: "fill-height" }, {
                    default: withCtx(() => [
                      createVNode(VCardItem, null, {
                        prepend: withCtx(() => [
                          createVNode(VIcon, {
                            color: unref(mcpStore).isServerConnected(server) ? "success" : "warning"
                          }, {
                            default: withCtx(() => [
                              createTextVNode(toDisplayString(unref(mcpStore).isServerConnected(server) ? "mdi-server" : "mdi-server-off"), 1)
                            ]),
                            _: 2
                          }, 1032, ["color"])
                        ]),
                        default: withCtx(() => [
                          createVNode(VCardTitle, null, {
                            default: withCtx(() => [
                              createTextVNode(toDisplayString(server.name), 1)
                            ]),
                            _: 2
                          }, 1024),
                          createVNode(VCardSubtitle, null, {
                            default: withCtx(() => [
                              createTextVNode(toDisplayString(server.command ? `${server.command} ${server.args.join(" ")}` : server.url || "No endpoint"), 1)
                            ]),
                            _: 2
                          }, 1024)
                        ]),
                        _: 2
                      }, 1024),
                      createVNode(VCardText, null, {
                        default: withCtx(() => {
                          var _a;
                          return [
                            createBaseVNode("div", _hoisted_4, [
                              createVNode(VChip, {
                                color: unref(mcpStore).isServerConnected(server) ? "success" : "warning",
                                size: "small",
                                variant: "tonal"
                              }, {
                                default: withCtx(() => [
                                  createTextVNode(toDisplayString(unref(mcpStore).isServerConnected(server) ? "running" : "stopped"), 1)
                                ]),
                                _: 2
                              }, 1032, ["color"]),
                              createVNode(VChip, {
                                size: "small",
                                variant: "outlined"
                              }, {
                                default: withCtx(() => [
                                  createTextVNode(toDisplayString(server.transport), 1)
                                ]),
                                _: 2
                              }, 1024),
                              server.scope ? (openBlock(), createBlock(VChip, {
                                key: 0,
                                size: "small",
                                variant: "outlined"
                              }, {
                                default: withCtx(() => [
                                  createTextVNode(toDisplayString(server.scope), 1)
                                ]),
                                _: 2
                              }, 1024)) : createCommentVNode("", true)
                            ]),
                            server.env && Object.keys(server.env).length ? (openBlock(), createElementBlock("div", _hoisted_5, [
                              _cache[16] || (_cache[16] = createBaseVNode("div", { class: "text-caption text-medium-emphasis mb-1" }, "Environment:", -1)),
                              (openBlock(true), createElementBlock(Fragment, null, renderList(server.env, (val, key) => {
                                return openBlock(), createElementBlock("div", {
                                  key,
                                  class: "text-caption"
                                }, [
                                  createBaseVNode("code", null, toDisplayString(key) + "=" + toDisplayString(val), 1)
                                ]);
                              }), 128))
                            ])) : createCommentVNode("", true),
                            ((_a = server.status) == null ? void 0 : _a.error) ? (openBlock(), createElementBlock("div", _hoisted_6, [
                              createBaseVNode("div", _hoisted_7, toDisplayString(server.status.error), 1)
                            ])) : createCommentVNode("", true)
                          ];
                        }),
                        _: 2
                      }, 1024),
                      createVNode(VCardActions, null, {
                        default: withCtx(() => [
                          createVNode(VBtn, {
                            variant: "tonal",
                            size: "small",
                            onClick: ($event) => testConnection(server.name),
                            loading: unref(mcpStore).testingServer === server.name
                          }, {
                            default: withCtx(() => [
                              createVNode(VIcon, { start: "" }, {
                                default: withCtx(() => [..._cache[17] || (_cache[17] = [
                                  createTextVNode("mdi-connection", -1)
                                ])]),
                                _: 1
                              }),
                              _cache[18] || (_cache[18] = createTextVNode(" Test ", -1))
                            ]),
                            _: 1
                          }, 8, ["onClick", "loading"]),
                          createVNode(VSpacer),
                          createVNode(VBtn, {
                            icon: "mdi-delete",
                            variant: "text",
                            size: "small",
                            color: "error",
                            onClick: ($event) => confirmRemove(server)
                          }, null, 8, ["onClick"])
                        ]),
                        _: 2
                      }, 1024)
                    ]),
                    _: 2
                  }, 1024)
                ]),
                _: 2
              }, 1024);
            }), 128))
          ]),
          _: 1
        }),
        unref(mcpStore).loading ? (openBlock(), createBlock(VSkeletonLoader, {
          key: 0,
          type: "card@4"
        })) : createCommentVNode("", true),
        !unref(mcpStore).loading && unref(mcpStore).servers.length === 0 ? (openBlock(), createElementBlock("div", _hoisted_8, [
          createVNode(VIcon, {
            size: "64",
            color: "medium-emphasis"
          }, {
            default: withCtx(() => [..._cache[19] || (_cache[19] = [
              createTextVNode("mdi-server-off", -1)
            ])]),
            _: 1
          }),
          _cache[21] || (_cache[21] = createBaseVNode("p", { class: "text-medium-emphasis mt-4" }, "No MCP servers configured", -1)),
          createVNode(VBtn, {
            color: "primary",
            class: "mt-4",
            onClick: _cache[1] || (_cache[1] = ($event) => showAddDialog.value = true)
          }, {
            default: withCtx(() => [..._cache[20] || (_cache[20] = [
              createTextVNode("Add Server", -1)
            ])]),
            _: 1
          })
        ])) : createCommentVNode("", true),
        createVNode(VDialog, {
          modelValue: showAddDialog.value,
          "onUpdate:modelValue": _cache[9] || (_cache[9] = ($event) => showAddDialog.value = $event),
          "max-width": "600"
        }, {
          default: withCtx(() => [
            createVNode(VCard, null, {
              default: withCtx(() => [
                createVNode(VCardTitle, null, {
                  default: withCtx(() => [..._cache[22] || (_cache[22] = [
                    createTextVNode("Add MCP Server", -1)
                  ])]),
                  _: 1
                }),
                createVNode(VCardText, null, {
                  default: withCtx(() => [
                    createVNode(VTextField, {
                      modelValue: addForm.value.name,
                      "onUpdate:modelValue": _cache[2] || (_cache[2] = ($event) => addForm.value.name = $event),
                      label: "Server Name",
                      variant: "outlined",
                      class: "mb-3"
                    }, null, 8, ["modelValue"]),
                    createVNode(VSelect, {
                      modelValue: addForm.value.transport,
                      "onUpdate:modelValue": _cache[3] || (_cache[3] = ($event) => addForm.value.transport = $event),
                      label: "Transport",
                      items: ["stdio", "sse"],
                      variant: "outlined",
                      class: "mb-3"
                    }, null, 8, ["modelValue"]),
                    createVNode(VTextField, {
                      modelValue: addForm.value.command,
                      "onUpdate:modelValue": _cache[4] || (_cache[4] = ($event) => addForm.value.command = $event),
                      label: "Command",
                      variant: "outlined",
                      class: "mb-3"
                    }, null, 8, ["modelValue"]),
                    createVNode(VTextField, {
                      modelValue: addForm.value.argsText,
                      "onUpdate:modelValue": _cache[5] || (_cache[5] = ($event) => addForm.value.argsText = $event),
                      label: "Arguments (comma separated)",
                      variant: "outlined",
                      class: "mb-3"
                    }, null, 8, ["modelValue"]),
                    createVNode(VTextField, {
                      modelValue: addForm.value.url,
                      "onUpdate:modelValue": _cache[6] || (_cache[6] = ($event) => addForm.value.url = $event),
                      label: "URL (for SSE)",
                      variant: "outlined",
                      class: "mb-3"
                    }, null, 8, ["modelValue"]),
                    createVNode(VSelect, {
                      modelValue: addForm.value.scope,
                      "onUpdate:modelValue": _cache[7] || (_cache[7] = ($event) => addForm.value.scope = $event),
                      label: "Scope",
                      items: ["local", "project", "user"],
                      variant: "outlined",
                      class: "mb-3"
                    }, null, 8, ["modelValue"])
                  ]),
                  _: 1
                }),
                createVNode(VCardActions, null, {
                  default: withCtx(() => [
                    createVNode(VSpacer),
                    createVNode(VBtn, {
                      variant: "text",
                      onClick: _cache[8] || (_cache[8] = ($event) => showAddDialog.value = false)
                    }, {
                      default: withCtx(() => [..._cache[23] || (_cache[23] = [
                        createTextVNode("Cancel", -1)
                      ])]),
                      _: 1
                    }),
                    createVNode(VBtn, {
                      color: "primary",
                      onClick: addServer
                    }, {
                      default: withCtx(() => [..._cache[24] || (_cache[24] = [
                        createTextVNode("Add", -1)
                      ])]),
                      _: 1
                    })
                  ]),
                  _: 1
                })
              ]),
              _: 1
            })
          ]),
          _: 1
        }, 8, ["modelValue"]),
        createVNode(VDialog, {
          modelValue: removeDialog.value,
          "onUpdate:modelValue": _cache[11] || (_cache[11] = ($event) => removeDialog.value = $event),
          "max-width": "400"
        }, {
          default: withCtx(() => [
            createVNode(VCard, null, {
              default: withCtx(() => [
                createVNode(VCardTitle, null, {
                  default: withCtx(() => [..._cache[25] || (_cache[25] = [
                    createTextVNode("Remove Server", -1)
                  ])]),
                  _: 1
                }),
                createVNode(VCardText, null, {
                  default: withCtx(() => {
                    var _a;
                    return [
                      _cache[26] || (_cache[26] = createTextVNode(" Remove ", -1)),
                      createBaseVNode("strong", null, toDisplayString((_a = removeTarget.value) == null ? void 0 : _a.name), 1),
                      _cache[27] || (_cache[27] = createTextVNode("? ", -1))
                    ];
                  }),
                  _: 1
                }),
                createVNode(VCardActions, null, {
                  default: withCtx(() => [
                    createVNode(VSpacer),
                    createVNode(VBtn, {
                      variant: "text",
                      onClick: _cache[10] || (_cache[10] = ($event) => removeDialog.value = false)
                    }, {
                      default: withCtx(() => [..._cache[28] || (_cache[28] = [
                        createTextVNode("Cancel", -1)
                      ])]),
                      _: 1
                    }),
                    createVNode(VBtn, {
                      color: "error",
                      onClick: handleRemove
                    }, {
                      default: withCtx(() => [..._cache[29] || (_cache[29] = [
                        createTextVNode("Remove", -1)
                      ])]),
                      _: 1
                    })
                  ]),
                  _: 1
                })
              ]),
              _: 1
            })
          ]),
          _: 1
        }, 8, ["modelValue"])
      ]);
    };
  }
});
const McpPage = /* @__PURE__ */ _export_sfc(_sfc_main, [["__scopeId", "data-v-aa2e6708"]]);
export {
  McpPage as default
};
