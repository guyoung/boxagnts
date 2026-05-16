import { P as defineStore, $ as ref, G as api, O as defineComponent, a5 as useAppStore, Y as onMounted, L as createElementBlock, I as createBaseVNode, N as createVNode, ac as withCtx, p as VIcon, c as VBtn, x as VRow, a4 as unref, J as createBlock, K as createCommentVNode, m as VDialog, Z as openBlock, M as createTextVNode, F as Fragment, a0 as renderList, d as VCard, f as VCardItem, i as VCardTitle, a2 as toDisplayString, g as VCardSubtitle, h as VCardText, q as VList, s as VListItem, u as VListItemTitle, k as VChip, e as VCardActions, B as VSwitch, A as VSpacer, l as VCol, C as VTextField, D as VTextarea, v as VMenu, w as VProgressCircular, r as VListGroup, af as withModifiers, T as mergeProps, y as VSelect, H as computed, z as VSkeletonLoader } from "./index-orSBHcqs.js";
const useSiteStore = defineStore("sites", () => {
  const sites = ref([]);
  const loading = ref(false);
  async function fetchSites() {
    loading.value = true;
    try {
      sites.value = await api.getSites();
    } catch (e) {
      console.error("Failed to fetch sites:", e);
      sites.value = [];
    } finally {
      loading.value = false;
    }
  }
  async function addSite(data) {
    const site = await api.createSite(data);
    sites.value.push(site);
    return site;
  }
  async function updateSite(id, data) {
    const site = await api.updateSite(id, data);
    const idx = sites.value.findIndex((s) => s.id === id);
    if (idx >= 0) {
      sites.value[idx] = site;
    }
    return site;
  }
  async function removeSite(id) {
    await api.deleteSite(id);
    sites.value = sites.value.filter((s) => s.id !== id);
  }
  return {
    sites,
    loading,
    fetchSites,
    addSite,
    updateSite,
    removeSite
  };
});
const _hoisted_1 = { class: "d-flex align-center justify-space-between mb-6" };
const _hoisted_2 = { class: "d-flex align-center" };
const _hoisted_3 = {
  key: 0,
  class: "text-body-2 mb-3"
};
const _hoisted_4 = { class: "text-caption" };
const _hoisted_5 = { class: "text-caption" };
const _hoisted_6 = { class: "text-caption" };
const _hoisted_7 = { class: "text-caption" };
const _hoisted_8 = { class: "d-flex align-center gap-2 mt-2" };
const _hoisted_9 = {
  key: 1,
  class: "text-center py-12"
};
const _sfc_main = /* @__PURE__ */ defineComponent({
  __name: "SitesPage",
  setup(__props) {
    const siteStore = useSiteStore();
    const appStore = useAppStore();
    const showDialog = ref(false);
    const editingSite = ref(null);
    const saving = ref(false);
    const deleteDialog = ref(false);
    const deleteTarget = ref(null);
    const deleting = ref(false);
    const folders = ref([]);
    const loadingFolders = ref(false);
    const pathMenuOpen = ref(false);
    const componentOptions = ["boxed_static_server_component.wasm"];
    const defaultForm = () => ({
      name: "",
      title: "",
      description: "",
      path: "",
      entry_point: null,
      component: "boxed_static_server_component.wasm",
      enabled: true,
      enable_auth: null,
      auth_user: null,
      auth_pass: null
    });
    const form = ref(defaultForm());
    function buildTree(folderItems) {
      const root = [];
      const pathMap = /* @__PURE__ */ new Map();
      folderItems.forEach((item) => {
        const node = { name: item.name, path: item.path, children: [] };
        pathMap.set(item.path, node);
      });
      folderItems.forEach((item) => {
        const node = pathMap.get(item.path);
        const parts = item.path.split("/");
        if (parts.length === 1) {
          root.push(node);
        } else {
          const parentPath = parts.slice(0, -1).join("/");
          const parent = pathMap.get(parentPath);
          if (parent) {
            parent.children.push(node);
          } else {
            root.push(node);
          }
        }
      });
      return root;
    }
    const folderTree = computed(() => buildTree(folders.value));
    async function fetchFolders() {
      loadingFolders.value = true;
      try {
        folders.value = await api.getRootSubFolders();
      } catch (e) {
        console.error("Failed to fetch folders:", e);
        folders.value = [];
      } finally {
        loadingFolders.value = false;
      }
    }
    function openAddDialog() {
      editingSite.value = null;
      form.value = defaultForm();
      showDialog.value = true;
      fetchFolders();
    }
    function openEditDialog(site) {
      editingSite.value = site;
      form.value = {
        name: site.name,
        title: site.title,
        description: site.description,
        path: site.path,
        entry_point: site.entry_point,
        component: site.component,
        enabled: site.enabled,
        enable_auth: site.enable_auth,
        auth_user: site.auth_user,
        auth_pass: site.auth_pass
      };
      showDialog.value = true;
      fetchFolders();
    }
    function selectPath(path) {
      form.value.path = path;
    }
    async function handleSave() {
      saving.value = true;
      try {
        if (editingSite.value) {
          await siteStore.updateSite(editingSite.value.id, { ...form.value });
          appStore.showMessage("Site updated", "success");
        } else {
          await siteStore.addSite({ ...form.value });
          appStore.showMessage("Site created", "success");
        }
        showDialog.value = false;
        form.value = defaultForm();
        editingSite.value = null;
        await siteStore.fetchSites();
      } catch {
        appStore.showMessage("Failed to save site", "error");
      } finally {
        saving.value = false;
      }
    }
    function confirmRemove(site) {
      deleteTarget.value = site;
      deleteDialog.value = true;
    }
    async function handleDelete() {
      if (!deleteTarget.value) return;
      deleting.value = true;
      try {
        await siteStore.removeSite(deleteTarget.value.id);
        appStore.showMessage("Site deleted", "success");
        await siteStore.fetchSites();
      } catch {
        appStore.showMessage("Failed to delete site", "error");
      } finally {
        deleting.value = false;
        deleteDialog.value = false;
        deleteTarget.value = null;
      }
    }
    async function handleToggleEnabled(site, enabled) {
      try {
        await siteStore.updateSite(site.id, { enabled });
        appStore.showMessage(enabled ? "Site enabled" : "Site disabled", "success");
        await siteStore.fetchSites();
      } catch {
        appStore.showMessage("Failed to update site", "error");
      }
    }
    onMounted(() => {
      siteStore.fetchSites();
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
              default: withCtx(() => [..._cache[15] || (_cache[15] = [
                createTextVNode("mdi-web", -1)
              ])]),
              _: 1
            }),
            _cache[16] || (_cache[16] = createBaseVNode("h1", { class: "text-h4 font-weight-bold" }, "Sites", -1))
          ]),
          createVNode(VBtn, {
            color: "primary",
            "prepend-icon": "mdi-plus",
            onClick: openAddDialog
          }, {
            default: withCtx(() => [..._cache[17] || (_cache[17] = [
              createTextVNode(" Add Site ", -1)
            ])]),
            _: 1
          })
        ]),
        createVNode(VRow, null, {
          default: withCtx(() => [
            (openBlock(true), createElementBlock(Fragment, null, renderList(unref(siteStore).sites, (site) => {
              return openBlock(), createBlock(VCol, {
                cols: "12",
                md: "6",
                lg: "4",
                key: site.id
              }, {
                default: withCtx(() => [
                  createVNode(VCard, { class: "fill-height" }, {
                    default: withCtx(() => [
                      createVNode(VCardItem, null, {
                        prepend: withCtx(() => [
                          createVNode(VIcon, {
                            color: site.enable_auth ? "warning" : "success",
                            size: "28"
                          }, {
                            default: withCtx(() => [
                              createTextVNode(toDisplayString(site.enable_auth ? "mdi-web-lock" : "mdi-web"), 1)
                            ]),
                            _: 2
                          }, 1032, ["color"])
                        ]),
                        default: withCtx(() => [
                          createVNode(VCardTitle, null, {
                            default: withCtx(() => [
                              createTextVNode(toDisplayString(site.title || site.name), 1)
                            ]),
                            _: 2
                          }, 1024),
                          createVNode(VCardSubtitle, null, {
                            default: withCtx(() => [
                              createTextVNode(toDisplayString(site.name), 1)
                            ]),
                            _: 2
                          }, 1024)
                        ]),
                        _: 2
                      }, 1024),
                      createVNode(VCardText, null, {
                        default: withCtx(() => [
                          site.description ? (openBlock(), createElementBlock("div", _hoisted_3, toDisplayString(site.description), 1)) : createCommentVNode("", true),
                          createVNode(VList, {
                            density: "compact",
                            class: "pa-0"
                          }, {
                            default: withCtx(() => [
                              createVNode(VListItem, null, {
                                prepend: withCtx(() => [
                                  createVNode(VIcon, {
                                    size: "16",
                                    color: "medium-emphasis"
                                  }, {
                                    default: withCtx(() => [..._cache[18] || (_cache[18] = [
                                      createTextVNode("mdi-folder", -1)
                                    ])]),
                                    _: 1
                                  })
                                ]),
                                append: withCtx(() => [
                                  createBaseVNode("code", _hoisted_4, toDisplayString(site.path), 1)
                                ]),
                                default: withCtx(() => [
                                  createVNode(VListItemTitle, { class: "text-caption" }, {
                                    default: withCtx(() => [..._cache[19] || (_cache[19] = [
                                      createTextVNode("Path", -1)
                                    ])]),
                                    _: 1
                                  })
                                ]),
                                _: 2
                              }, 1024),
                              site.entry_point ? (openBlock(), createBlock(VListItem, { key: 0 }, {
                                prepend: withCtx(() => [
                                  createVNode(VIcon, {
                                    size: "16",
                                    color: "medium-emphasis"
                                  }, {
                                    default: withCtx(() => [..._cache[20] || (_cache[20] = [
                                      createTextVNode("mdi-file-code", -1)
                                    ])]),
                                    _: 1
                                  })
                                ]),
                                append: withCtx(() => [
                                  createBaseVNode("code", _hoisted_5, toDisplayString(site.entry_point), 1)
                                ]),
                                default: withCtx(() => [
                                  createVNode(VListItemTitle, { class: "text-caption" }, {
                                    default: withCtx(() => [..._cache[21] || (_cache[21] = [
                                      createTextVNode("Entry Point", -1)
                                    ])]),
                                    _: 1
                                  })
                                ]),
                                _: 2
                              }, 1024)) : createCommentVNode("", true),
                              site.component ? (openBlock(), createBlock(VListItem, { key: 1 }, {
                                prepend: withCtx(() => [
                                  createVNode(VIcon, {
                                    size: "16",
                                    color: "medium-emphasis"
                                  }, {
                                    default: withCtx(() => [..._cache[22] || (_cache[22] = [
                                      createTextVNode("mdi-puzzle", -1)
                                    ])]),
                                    _: 1
                                  })
                                ]),
                                append: withCtx(() => [
                                  createBaseVNode("code", _hoisted_6, toDisplayString(site.component), 1)
                                ]),
                                default: withCtx(() => [
                                  createVNode(VListItemTitle, { class: "text-caption" }, {
                                    default: withCtx(() => [..._cache[23] || (_cache[23] = [
                                      createTextVNode("Component", -1)
                                    ])]),
                                    _: 1
                                  })
                                ]),
                                _: 2
                              }, 1024)) : createCommentVNode("", true),
                              site.enable_auth && site.auth_user ? (openBlock(), createBlock(VListItem, { key: 2 }, {
                                prepend: withCtx(() => [
                                  createVNode(VIcon, {
                                    size: "16",
                                    color: "medium-emphasis"
                                  }, {
                                    default: withCtx(() => [..._cache[24] || (_cache[24] = [
                                      createTextVNode("mdi-account", -1)
                                    ])]),
                                    _: 1
                                  })
                                ]),
                                append: withCtx(() => [
                                  createBaseVNode("span", _hoisted_7, toDisplayString(site.auth_user), 1)
                                ]),
                                default: withCtx(() => [
                                  createVNode(VListItemTitle, { class: "text-caption" }, {
                                    default: withCtx(() => [..._cache[25] || (_cache[25] = [
                                      createTextVNode("Auth User", -1)
                                    ])]),
                                    _: 1
                                  })
                                ]),
                                _: 2
                              }, 1024)) : createCommentVNode("", true)
                            ]),
                            _: 2
                          }, 1024),
                          createBaseVNode("div", _hoisted_8, [
                            createVNode(VChip, {
                              color: site.enable_auth ? "warning" : "success",
                              size: "x-small",
                              variant: "tonal"
                            }, {
                              default: withCtx(() => [
                                createTextVNode(toDisplayString(site.enable_auth ? "Auth Enabled" : "Public"), 1)
                              ]),
                              _: 2
                            }, 1032, ["color"])
                          ])
                        ]),
                        _: 2
                      }, 1024),
                      createVNode(VCardActions, null, {
                        default: withCtx(() => [
                          createVNode(VSwitch, {
                            "model-value": site.enabled,
                            label: site.enabled ? "Enabled" : "Disabled",
                            color: "success",
                            density: "compact",
                            "hide-details": "",
                            "onUpdate:modelValue": (v) => handleToggleEnabled(site, !!v)
                          }, null, 8, ["model-value", "label", "onUpdate:modelValue"]),
                          createVNode(VSpacer),
                          createVNode(VBtn, {
                            variant: "tonal",
                            size: "small",
                            "prepend-icon": "mdi-pencil",
                            onClick: ($event) => openEditDialog(site)
                          }, {
                            default: withCtx(() => [..._cache[26] || (_cache[26] = [
                              createTextVNode(" Edit ", -1)
                            ])]),
                            _: 1
                          }, 8, ["onClick"]),
                          createVNode(VSpacer),
                          createVNode(VBtn, {
                            icon: "mdi-delete",
                            variant: "text",
                            size: "small",
                            color: "error",
                            onClick: ($event) => confirmRemove(site)
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
        unref(siteStore).loading ? (openBlock(), createBlock(VSkeletonLoader, {
          key: 0,
          type: "card@3"
        })) : createCommentVNode("", true),
        !unref(siteStore).loading && unref(siteStore).sites.length === 0 ? (openBlock(), createElementBlock("div", _hoisted_9, [
          createVNode(VIcon, {
            size: "64",
            color: "medium-emphasis"
          }, {
            default: withCtx(() => [..._cache[27] || (_cache[27] = [
              createTextVNode("mdi-web-off", -1)
            ])]),
            _: 1
          }),
          _cache[29] || (_cache[29] = createBaseVNode("p", { class: "text-medium-emphasis mt-4" }, "No sites configured", -1)),
          createVNode(VBtn, {
            color: "primary",
            class: "mt-4",
            onClick: openAddDialog
          }, {
            default: withCtx(() => [..._cache[28] || (_cache[28] = [
              createTextVNode("Add Site", -1)
            ])]),
            _: 1
          })
        ])) : createCommentVNode("", true),
        createVNode(VDialog, {
          modelValue: showDialog.value,
          "onUpdate:modelValue": _cache[12] || (_cache[12] = ($event) => showDialog.value = $event),
          "max-width": "600"
        }, {
          default: withCtx(() => [
            createVNode(VCard, null, {
              default: withCtx(() => [
                createVNode(VCardTitle, null, {
                  default: withCtx(() => [
                    createTextVNode(toDisplayString(editingSite.value ? "Edit Site" : "Add Site"), 1)
                  ]),
                  _: 1
                }),
                createVNode(VCardText, null, {
                  default: withCtx(() => [
                    createVNode(VRow, { dense: "" }, {
                      default: withCtx(() => [
                        createVNode(VCol, {
                          cols: "12",
                          md: "6"
                        }, {
                          default: withCtx(() => [
                            createVNode(VTextField, {
                              modelValue: form.value.name,
                              "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => form.value.name = $event),
                              label: "Name",
                              variant: "outlined",
                              placeholder: "my-site",
                              hint: "Unique identifier for the site",
                              "persistent-hint": ""
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, {
                          cols: "12",
                          md: "6"
                        }, {
                          default: withCtx(() => [
                            createVNode(VTextField, {
                              modelValue: form.value.title,
                              "onUpdate:modelValue": _cache[1] || (_cache[1] = ($event) => form.value.title = $event),
                              label: "Title",
                              variant: "outlined",
                              placeholder: "My Site"
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, { cols: "12" }, {
                          default: withCtx(() => [
                            createVNode(VTextarea, {
                              modelValue: form.value.description,
                              "onUpdate:modelValue": _cache[2] || (_cache[2] = ($event) => form.value.description = $event),
                              label: "Description",
                              variant: "outlined",
                              rows: "2",
                              placeholder: "Site description..."
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, {
                          cols: "12",
                          md: "6"
                        }, {
                          default: withCtx(() => [
                            createVNode(VMenu, {
                              modelValue: pathMenuOpen.value,
                              "onUpdate:modelValue": _cache[4] || (_cache[4] = ($event) => pathMenuOpen.value = $event),
                              activator: "parent",
                              location: "bottom"
                            }, {
                              activator: withCtx(({ props }) => [
                                createVNode(VTextField, mergeProps({
                                  modelValue: form.value.path,
                                  "onUpdate:modelValue": _cache[3] || (_cache[3] = ($event) => form.value.path = $event),
                                  label: "Path",
                                  variant: "outlined",
                                  placeholder: "Select folder",
                                  "append-icon": "mdi-folder",
                                  readonly: ""
                                }, props), null, 16, ["modelValue"])
                              ]),
                              default: withCtx(() => [
                                loadingFolders.value ? (openBlock(), createBlock(VList, { key: 0 }, {
                                  default: withCtx(() => [
                                    createVNode(VProgressCircular, {
                                      indeterminate: "",
                                      color: "primary"
                                    })
                                  ]),
                                  _: 1
                                })) : folderTree.value.length === 0 ? (openBlock(), createBlock(VList, { key: 1 }, {
                                  default: withCtx(() => [
                                    createVNode(VListItem, { title: "No folders available" })
                                  ]),
                                  _: 1
                                })) : (openBlock(), createBlock(VList, {
                                  key: 2,
                                  density: "compact"
                                }, {
                                  default: withCtx(() => [
                                    (openBlock(true), createElementBlock(Fragment, null, renderList(folderTree.value, (node) => {
                                      return openBlock(), createBlock(VListGroup, {
                                        key: node.path,
                                        "model-value": node.path.includes(form.value.path),
                                        "prepend-icon": "mdi-folder"
                                      }, {
                                        activator: withCtx(({ props }) => [
                                          createVNode(VListItem, mergeProps({ ref_for: true }, props, {
                                            title: node.name,
                                            active: form.value.path === node.path,
                                            color: form.value.path === node.path ? "primary" : void 0,
                                            onClick: withModifiers(($event) => {
                                              selectPath(node.path);
                                              pathMenuOpen.value = false;
                                            }, ["stop"])
                                          }), null, 16, ["title", "active", "color", "onClick"])
                                        ]),
                                        default: withCtx(() => [
                                          createVNode(VList, null, {
                                            default: withCtx(() => [
                                              (openBlock(true), createElementBlock(Fragment, null, renderList(node.children, (child) => {
                                                return openBlock(), createElementBlock(Fragment, {
                                                  key: child.path
                                                }, [
                                                  child.children.length > 0 ? (openBlock(), createBlock(VListGroup, {
                                                    key: 0,
                                                    "model-value": child.path.includes(form.value.path),
                                                    "prepend-icon": "mdi-folder"
                                                  }, {
                                                    activator: withCtx(({ props }) => [
                                                      createVNode(VListItem, mergeProps({ ref_for: true }, props, {
                                                        title: child.name,
                                                        active: form.value.path === child.path,
                                                        color: form.value.path === child.path ? "primary" : void 0,
                                                        onClick: withModifiers(($event) => {
                                                          selectPath(child.path);
                                                          pathMenuOpen.value = false;
                                                        }, ["stop"])
                                                      }), null, 16, ["title", "active", "color", "onClick"])
                                                    ]),
                                                    default: withCtx(() => [
                                                      createVNode(VList, null, {
                                                        default: withCtx(() => [
                                                          (openBlock(true), createElementBlock(Fragment, null, renderList(child.children, (subChild) => {
                                                            return openBlock(), createBlock(VListItem, {
                                                              key: subChild.path,
                                                              title: subChild.name,
                                                              "prepend-icon": "mdi-folder-outline",
                                                              active: form.value.path === subChild.path,
                                                              color: form.value.path === subChild.path ? "primary" : void 0,
                                                              onClick: withModifiers(($event) => {
                                                                selectPath(subChild.path);
                                                                pathMenuOpen.value = false;
                                                              }, ["stop"])
                                                            }, null, 8, ["title", "active", "color", "onClick"]);
                                                          }), 128))
                                                        ]),
                                                        _: 2
                                                      }, 1024)
                                                    ]),
                                                    _: 2
                                                  }, 1032, ["model-value"])) : (openBlock(), createBlock(VListItem, {
                                                    key: 1,
                                                    title: child.name,
                                                    "prepend-icon": "mdi-folder-outline",
                                                    active: form.value.path === child.path,
                                                    color: form.value.path === child.path ? "primary" : void 0,
                                                    onClick: withModifiers(($event) => {
                                                      selectPath(child.path);
                                                      pathMenuOpen.value = false;
                                                    }, ["stop"])
                                                  }, null, 8, ["title", "active", "color", "onClick"]))
                                                ], 64);
                                              }), 128))
                                            ]),
                                            _: 2
                                          }, 1024)
                                        ]),
                                        _: 2
                                      }, 1032, ["model-value"]);
                                    }), 128))
                                  ]),
                                  _: 1
                                }))
                              ]),
                              _: 1
                            }, 8, ["modelValue"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, {
                          cols: "12",
                          md: "6"
                        }, {
                          default: withCtx(() => [
                            createVNode(VTextField, {
                              modelValue: form.value.entry_point,
                              "onUpdate:modelValue": _cache[5] || (_cache[5] = ($event) => form.value.entry_point = $event),
                              label: "Entry Point",
                              variant: "outlined",
                              placeholder: "index.html"
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, {
                          cols: "12",
                          md: "6"
                        }, {
                          default: withCtx(() => [
                            createVNode(VSelect, {
                              modelValue: form.value.component,
                              "onUpdate:modelValue": _cache[6] || (_cache[6] = ($event) => form.value.component = $event),
                              label: "Component",
                              variant: "outlined",
                              items: componentOptions,
                              readonly: true,
                              "persistent-hint": "",
                              hint: "Only one option available"
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, { cols: "12" }, {
                          default: withCtx(() => [
                            createVNode(VSwitch, {
                              modelValue: form.value.enable_auth,
                              "onUpdate:modelValue": _cache[7] || (_cache[7] = ($event) => form.value.enable_auth = $event),
                              label: "Enable Authentication",
                              color: "primary",
                              "hide-details": "",
                              class: "mt-1"
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, { cols: "12" }, {
                          default: withCtx(() => [
                            createVNode(VSwitch, {
                              modelValue: form.value.enabled,
                              "onUpdate:modelValue": _cache[8] || (_cache[8] = ($event) => form.value.enabled = $event),
                              label: "Enabled",
                              color: "success",
                              "hide-details": ""
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, {
                          cols: "12",
                          md: "6"
                        }, {
                          default: withCtx(() => [
                            createVNode(VTextField, {
                              modelValue: form.value.auth_user,
                              "onUpdate:modelValue": _cache[9] || (_cache[9] = ($event) => form.value.auth_user = $event),
                              label: "Auth User",
                              variant: "outlined",
                              placeholder: "admin",
                              disabled: !form.value.enable_auth
                            }, null, 8, ["modelValue", "disabled"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, {
                          cols: "12",
                          md: "6"
                        }, {
                          default: withCtx(() => [
                            createVNode(VTextField, {
                              modelValue: form.value.auth_pass,
                              "onUpdate:modelValue": _cache[10] || (_cache[10] = ($event) => form.value.auth_pass = $event),
                              label: "Auth Password",
                              variant: "outlined",
                              type: "password",
                              placeholder: "Enter password",
                              disabled: !form.value.enable_auth
                            }, null, 8, ["modelValue", "disabled"])
                          ]),
                          _: 1
                        })
                      ]),
                      _: 1
                    })
                  ]),
                  _: 1
                }),
                createVNode(VCardActions, null, {
                  default: withCtx(() => [
                    createVNode(VSpacer),
                    createVNode(VBtn, {
                      variant: "text",
                      onClick: _cache[11] || (_cache[11] = ($event) => showDialog.value = false)
                    }, {
                      default: withCtx(() => [..._cache[30] || (_cache[30] = [
                        createTextVNode("Cancel", -1)
                      ])]),
                      _: 1
                    }),
                    createVNode(VBtn, {
                      color: "primary",
                      onClick: handleSave,
                      loading: saving.value,
                      disabled: !form.value.name.trim()
                    }, {
                      default: withCtx(() => [
                        createTextVNode(toDisplayString(editingSite.value ? "Update" : "Create"), 1)
                      ]),
                      _: 1
                    }, 8, ["loading", "disabled"])
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
          modelValue: deleteDialog.value,
          "onUpdate:modelValue": _cache[14] || (_cache[14] = ($event) => deleteDialog.value = $event),
          "max-width": "400"
        }, {
          default: withCtx(() => [
            createVNode(VCard, null, {
              default: withCtx(() => [
                createVNode(VCardTitle, null, {
                  default: withCtx(() => [..._cache[31] || (_cache[31] = [
                    createTextVNode("Delete Site", -1)
                  ])]),
                  _: 1
                }),
                createVNode(VCardText, null, {
                  default: withCtx(() => {
                    var _a, _b;
                    return [
                      createBaseVNode("p", null, [
                        _cache[32] || (_cache[32] = createTextVNode(" Are you sure you want to delete ", -1)),
                        createBaseVNode("strong", null, toDisplayString(((_a = deleteTarget.value) == null ? void 0 : _a.title) || ((_b = deleteTarget.value) == null ? void 0 : _b.name)), 1),
                        _cache[33] || (_cache[33] = createTextVNode("? ", -1))
                      ]),
                      _cache[34] || (_cache[34] = createBaseVNode("p", { class: "text-caption text-error mt-2" }, "This action cannot be undone.", -1))
                    ];
                  }),
                  _: 1
                }),
                createVNode(VCardActions, null, {
                  default: withCtx(() => [
                    createVNode(VSpacer),
                    createVNode(VBtn, {
                      variant: "text",
                      onClick: _cache[13] || (_cache[13] = ($event) => deleteDialog.value = false)
                    }, {
                      default: withCtx(() => [..._cache[35] || (_cache[35] = [
                        createTextVNode("Cancel", -1)
                      ])]),
                      _: 1
                    }),
                    createVNode(VBtn, {
                      color: "error",
                      onClick: handleDelete,
                      loading: deleting.value
                    }, {
                      default: withCtx(() => [..._cache[36] || (_cache[36] = [
                        createTextVNode("Delete", -1)
                      ])]),
                      _: 1
                    }, 8, ["loading"])
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
export {
  _sfc_main as default
};
